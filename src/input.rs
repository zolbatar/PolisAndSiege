use raylib::prelude::{MouseButton};
use raylib::ffi::{GetMouseDelta, GetMouseWheelMove, IsMouseButtonPressed, IsMouseButtonReleased};
use crate::app_state::AppState;

static mut PANNING: bool = false;
static THRESHOLD: f32 = 64.0;

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
        PANNING = true; // Start panning when right button is pressed
    } else if IsMouseButtonReleased(MouseButton::MOUSE_BUTTON_RIGHT as i32) {
        PANNING = false; // Stop panning when button is released
    }

    if PANNING {
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
        app_state.camera.zoom = app_state.camera.zoom.clamp(1.0, 15.0);
    }
}
