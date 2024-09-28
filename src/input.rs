use std::os::raw::c_int;
use raylib::prelude::{MouseButton};
use raylib::ffi::{GetMouseDelta, GetMouseWheelMove, IsKeyPressed, IsMouseButtonPressed, IsMouseButtonReleased};
use raylib::prelude::KeyboardKey::KEY_L;
use crate::app_state::AppState;

const THRESHOLD: f32 = 64.0;

fn vector2_subtract(v1: raylib::core::math::Vector2, v2: raylib::core::math::Vector2) -> raylib::core::math::Vector2 {
    raylib::core::math::Vector2 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
    }
}

fn vector2_add(v1: raylib::core::math::Vector2, v2: raylib::core::math::Vector2) -> raylib::core::math::Vector2 {
    raylib::core::math::Vector2 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
    }
}
pub unsafe fn handle_input(app_state: &mut AppState) {

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
    }
}
