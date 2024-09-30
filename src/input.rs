use sdl2::mouse::{MouseButton, MouseWheelDirection};
use crate::app_state::AppState;

const THRESHOLD: i32 = 64;

pub fn handle_mouse_wheel(app_state: &mut AppState, direction: MouseWheelDirection, precise_y: f32) {
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

/*
    // Mouse inputs
    if IsMouseButtonPressed(MouseButton::MOUSE_BUTTON_RIGHT as i32) {
        app_state.panning = true; // Start panning when right button is pressed
    } else if IsMouseButtonReleased(MouseButton::MOUSE_BUTTON_RIGHT as i32) {
        app_state.panning = false; // Stop panning when button is released
    }

    if app_state.panning {
        // Calculate mouse movement delta
        let mut delta = GetMouseDelta();
        if delta.x.abs() < THRESHOLD && delta.y.abs() < THRESHOLD {

            // Scale the delta by the inverse of the camera zoom level
            delta.x /= app_state.camera.zoom;
            delta.y /= app_state.camera.zoom;

            // Update camera target based on mouse movement
            app_state.camera.target = vector2_subtract(app_state.camera.target, raylib::core::math::Vector2::new(delta.x, delta.y));
        }
    }

    // Zoom handling
    if GetMouseWheelMove() != 0.0 {
        let delta = GetMouseWheelMove() * 0.25;
        app_state.camera.zoom += delta;
        app_state.camera.zoom = app_state.camera.zoom.clamp(crate::app_state::MIN_ZOOM, 15.0);
    }

    // Keys
    if IsKeyPressed(KEY_L as c_int) {
        app_state.show_labels = !app_state.show_labels;
    }*/
