mod app_state;
mod lib {
    pub mod skia;
    pub mod cbor;
}
mod input;
mod render;

mod model {
    pub mod city;
    pub mod location;
    pub mod territory;
    pub mod territory_polygon;
    pub mod math;
    pub mod connection;
}

use crate::input::{handle_mouse_button_down, handle_mouse_button_up, handle_mouse_motion, handle_mouse_wheel};
use crate::lib::cbor;
use crate::lib::skia::Skia;
use crate::render::render;
use app_state::AppState;
use sdl2::video::GLProfile;

fn main() {
    // Initialize SDL2
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // Set OpenGL attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3); // OpenGL 3.3

    // Create an SDL2 window
    let window = video_subsystem
        .window("Simulation", 1600, 900)
        .opengl()
        .resizable()
        .allow_highdpi()
        .build()
        .unwrap();

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

    // Create an AppState instance using the new method
    dpi = dpi.floor();
    let mut app_state = AppState::new(&window, dpi);

    // Load CBOR data
    let territories = cbor::import(&mut app_state);
    app_state.territories = territories;

    // Skia and surfaces
    let mut skia = Skia::new(&app_state);
    unsafe { skia.flush(); }

    // Event pump for SDL2 events
    let mut event_pump = sdl.event_pump().unwrap();

    // Loop
    'running: loop {
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

        render(&mut skia, &mut app_state);
        window.gl_swap_window();
    }
}

