use crate::app_state::AppState;
use sdl2::mouse::{MouseButton, MouseWheelDirection};

const THRESHOLD: i32 = 64;

pub fn handle_mouse_wheel(app_state: &mut AppState, _direction: MouseWheelDirection, precise_y: f32) {
    let delta = precise_y * 0.25;
    app_state.zoom += delta;
    app_state.zoom = app_state.zoom.clamp(crate::app_state::MIN_ZOOM, 15.0);
}

pub fn handle_mouse_motion(app_state: &mut AppState, _x: i32, _y: i32, x_rel: i32, y_rel: i32) {
    if app_state.panning {
        // Calculate mouse movement delta
        if x_rel.abs() < THRESHOLD && y_rel.abs() < THRESHOLD {
            // Update camera target based on mouse movement
            app_state.target.x -= x_rel as f32 / app_state.zoom;
            app_state.target.y -= y_rel as f32 / app_state.zoom;
        }
    }
}

pub fn handle_mouse_button_down(app_state: &mut AppState, button: MouseButton) {
    if button == MouseButton::Right {
        app_state.panning = true;
    }
}

pub fn handle_mouse_button_up(app_state: &mut AppState, button: MouseButton) {
    if button == MouseButton::Right {
        app_state.panning = false;
    }
}
