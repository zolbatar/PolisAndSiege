use crate::app_state::{AppState, GameMode};
use crate::model::city::{Owner, SIZE};
use sdl2::mouse::{MouseButton, MouseWheelDirection};
use skia_safe::Point;

const THRESHOLD: i32 = 64;

pub fn handle_mouse_wheel(app_state: &mut AppState, _direction: MouseWheelDirection, precise_y: f32) {
    let delta = precise_y * 0.25;
    app_state.zoom += delta;
    app_state.zoom = app_state.zoom.clamp(crate::app_state::MIN_ZOOM, 15.0);
}

pub fn handle_mouse_motion(app_state: &mut AppState, x: i32, y: i32, x_rel: i32, y_rel: i32) {
    app_state.hover = Point::new(x as f32, y as f32);
    if app_state.panning {
        // Calculate mouse movement delta
        if x_rel.abs() < THRESHOLD && y_rel.abs() < THRESHOLD {
            // Update camera target based on mouse movement
            app_state.target.x -= x_rel as f32 / app_state.zoom;
            app_state.target.y -= y_rel as f32 / app_state.zoom;
        }
    } else {
        // Mouse over?
        let mut mp = Point::new(x as f32, y as f32);

        // Do reverse matrix transform
        mp.x -= app_state.gfx.half_width as f32;
        mp.y -= app_state.gfx.half_height as f32;
        mp.x /= app_state.zoom;
        mp.y /= app_state.zoom;
        mp.x += app_state.target.x;
        mp.y += app_state.target.y;

        if app_state.mode == GameMode::ArmyPlacement {
            app_state.selection.last_city_selection = None;
        } else {
            app_state.selection.last_city_hover = None;
        }
        for territory in &app_state.items.territories {
            for city in territory.1.lock().unwrap().cities.iter() {
                let delta = city.lock().unwrap().location.p - mp;
                let diff = (delta.x * delta.x + delta.y * delta.y).sqrt();
                if diff <= SIZE * app_state.zoom / app_state.gfx.dpi / 2.0 {
                    if app_state.mode == GameMode::ArmyPlacement {
                        app_state.selection.last_city_selection = Some(city.clone());
                    } else {
                        app_state.selection.last_city_hover = Some(city.clone());
                    }
                }
            }
        }
    }
}

pub fn handle_mouse_button_down(app_state: &mut AppState, button: MouseButton) {
    if button == MouseButton::Right {
        app_state.panning = true;
    } else if button == MouseButton::Left {
        match app_state.mode {
            GameMode::ArmyPlacement => {
                if let Some(city) = &app_state.selection.last_city_selection {
                    if city.lock().unwrap().owner == Owner::Player {
                        city.lock().unwrap().armies += 1;
                        app_state.armies_to_assign -= 1;
                        if app_state.armies_to_assign == 0 {
                            app_state.mode = GameMode::Game;
                        }
                    }
                }
            }
            GameMode::Game => {
                app_state.selection.last_city_selection = app_state.selection.last_city_hover.clone();
            }
            _ => {}
        }
    }
}

pub fn handle_mouse_button_up(app_state: &mut AppState, button: MouseButton) {
    if button == MouseButton::Right {
        app_state.panning = false;
    }
}
