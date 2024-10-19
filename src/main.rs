mod app_state;
mod lib {
    pub mod cbor;
    pub mod skia;
}
mod ai {
    pub mod army_placement;
    pub mod computer_turn;
    pub mod game;
    pub mod moves;
    pub mod possible_move;
}
mod input;

mod model {
    pub mod city;
    pub mod connection;
    pub mod location;
    pub mod math;
    pub mod player;
    pub mod profile;
    pub mod territory;
    pub mod territory_polygon;
    pub mod world_fixed;
    pub mod world_state;
}
mod render {
    pub mod army_placement;
    pub mod city_selection;
    pub mod entry;
    pub mod lower_panel;
    pub mod randomising;
    pub mod region_summary;
    pub mod surround;
    pub mod title_bar;
}

use crate::ai::computer_turn::computer_turn;
use crate::app_state::GameMode;
use crate::input::{handle_mouse_button_down, handle_mouse_button_up, handle_mouse_motion, handle_mouse_wheel};
use crate::lib::cbor;
use crate::lib::skia::Skia;
use crate::model::world_state::WorldState;
use crate::render::randomising::assign;
use app_state::AppState;
use sdl2::video::GLProfile;
use std::time::{Duration, Instant};

const ARMIES_PER_SIZE: f32 = 0.1;

fn main() {
    // Initialize SDL2
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // Set OpenGL attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3); // OpenGL 3.3

    // Create an SDL2 window
    let window = video_subsystem.window("Simulation", 1500, 900).opengl().allow_highdpi().build().unwrap();

    // Create an OpenGL context
    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();

    // Load OpenGL functions
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Get display index (typically 0 is the default display)
    let display_index = 0;

    // Get DPI information
    let mut dpi = 0.0;
    match video_subsystem.display_dpi(display_index) {
        Ok((ddpi, hdpi, vdpi)) => {
            println!("Diagonal DPI: {}", ddpi);
            println!("Horizontal DPI: {}", hdpi);
            println!("Vertical DPI: {}", vdpi);

            // Calculate scaling factor
            dpi = hdpi / 96.0; // 96 DPI is considered the default "normal" DPI
            println!("Scaling factor: {}", dpi);
        }
        Err(e) => {
            eprintln!("Could not get DPI information: {}", e);
        }
    }
    dpi = dpi.floor();

    // Create an AppState instance using the new method
    let mut app_state = AppState::new(&window, dpi, WorldState::default());
    app_state.world_state.current_player = Some(app_state.world_state.players[0].clone());

    // Skia and surfaces
    let mut skia = Skia::new(&app_state);
    unsafe {
        skia.flush();
    }

    // Load CBOR data
    cbor::import(&mut skia, &mut app_state);

    // Event pump for SDL2 events
    let mut event_pump = sdl.event_pump().unwrap();

    // Store the time of the previous frame and the last time we measured FPS
    let mut frame_count = 0;
    let mut last_fps_check = Instant::now();
    let fps_check_interval = Duration::from_secs(1); // Check FPS every second

    // Loop
    let start = Instant::now();
    'running: loop {
        // Measure the time it took to render the previous frame
        let current_time = Instant::now();
        app_state.phase = (current_time.duration_since(start).as_millis() as f32 / 250.0) % 2.0;

        // Increment the frame count
        frame_count += 1;

        // Calculate FPS every second
        if current_time - last_fps_check >= fps_check_interval {
            app_state.fps = frame_count as f64 / (fps_check_interval.as_secs_f64());

            // Reset frame count and last FPS check time
            frame_count = 0;
            last_fps_check = current_time;
        }

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }

                // Keys
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                sdl2::event::Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        // Handle arrow keys
                        sdl2::keyboard::Keycode::L => {
                            app_state.show_labels = true;
                        }
                        sdl2::keyboard::Keycode::M => {
                            app_state.show_labels = false;
                        }
                        sdl2::keyboard::Keycode::R => {
                            app_state.reset();
                        }

                        // Default case for other keys
                        _ => {}
                    }
                }

                // Mouse
                | sdl2::event::Event::MouseWheel { direction, precise_y, .. } => {
                    handle_mouse_wheel(&mut app_state, direction, precise_y);
                }
                | sdl2::event::Event::MouseMotion { x, y, xrel, yrel, .. } => {
                    handle_mouse_motion(&mut app_state, x, y, xrel, yrel);
                }
                | sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
                    handle_mouse_button_down(&mut app_state, mouse_btn);
                }
                | sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => {
                    handle_mouse_button_up(&mut app_state, mouse_btn);
                }
                _ => {}
            }
        }

        // Game loop
        if app_state.world_state.mode == GameMode::Randomising && app_state.world_state.current_player.is_some() {
            let diff = Instant::now() - app_state.selection.last_selection;
            if diff.as_millis() > app_state.selection.assign_speed {
                app_state.selection.last_selection = Instant::now();

                // Take top item
                if !app_state.world_fixed.cities_to_assign.is_empty() {
                    assign(&mut app_state);
                }
                next_turn(&mut app_state);
            }
        }

        // Waiting on AI action?
        if app_state.world_state.current_player.is_some()
            && !app_state.world_state.current_player.as_ref().unwrap().borrow().is_human()
        {
            match app_state.world_state.mode {
                GameMode::ArmyPlacement => computer_turn(&mut app_state),
                GameMode::Game => computer_turn(&mut app_state),
                _ => {}
            }
        }

        render::entry::main(&mut skia, &mut app_state);
        window.gl_swap_window();
    }
}

pub fn next_turn(app_state: &mut AppState) {
    let world_state = &mut app_state.world_state;
    let world_fixed = &app_state.world_fixed;
    world_state.update_scores(world_fixed);

    // Switch to next player
    let (turn_done, index) = {
        let mut index = world_state.current_player.as_ref().unwrap().borrow().index;
        index += 1;
        if index == world_state.players.len() {
            index = 0;
            (true, index)
        } else {
            (false, index)
        }
    };
    world_state.current_player = Some(world_state.players[index].clone());

    // Have we finished this phase?
    if turn_done {
        let current_player = world_state.get_current_player();
        match world_state.mode {
            GameMode::ArmyPlacement => {
                if current_player.borrow().armies_to_assign == 0 {
                    println!("All armies placed");
                    world_state.mode = GameMode::Game;
                }
            }
            GameMode::Randomising => {
                if world_fixed.cities_to_assign.is_empty() {
                    println!("All cities assigned");
                    world_state.mode = GameMode::ArmyPlacement;
                }
            }
            GameMode::Game => {
                // Time to update armies
                for player in &world_state.players {
                    let mut frac = 0.0f32;

                    for city in world_state.cities.iter() {
                        if city.borrow().owner.unwrap() == player.borrow().index {
                            frac += city.borrow().size as f32 * ARMIES_PER_SIZE;
                        }
                    }

                    player.borrow_mut().armies_to_assign_fractional += frac;
                    let frac_int = player.borrow().armies_to_assign_fractional as u32;
                    player.borrow_mut().armies_to_assign = frac_int;
                    player.borrow_mut().armies_to_assign_fractional -= frac_int as f32;
                }
                app_state.world_state.mode = GameMode::ArmyPlacement;

                // Need to calculate victory conditions
            }
            GameMode::End => {}
        }
    }
}
