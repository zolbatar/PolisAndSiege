mod app_state;
mod skia;
mod cbor;
mod model {
    pub mod city;
    pub mod location;
    pub mod territory;
    pub mod territory_polygon;
}

use crate::skia::{MySurface, Skia};
use app_state::AppState;
use raylib::color::Color;
use raylib::ffi::{DrawTexturePro, GetCurrentMonitor, GetMonitorHeight, GetMonitorWidth, GetMouseDelta, GetMouseWheelMove, IsMouseButtonPressed, IsMouseButtonReleased, Vector2};
use raylib::prelude::{MouseButton, RaylibDraw};
use raylib::{RaylibHandle, RaylibThread};
use skia_safe::{Paint, PaintStyle};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1600, 950)
        .title("Simulation")
        //        .undecorated()
        .build();

    // Load CBOR data
    let territories = cbor::import();

    // Create an AppState instance using the new method
    let mut app_state = AppState::new(&rl, territories);
    unsafe {
        let monitor = GetCurrentMonitor();
        println!("Native resolution: {} x {} ({} DPI)", GetMonitorWidth(monitor), GetMonitorHeight(monitor), app_state.dpi);
    }
    println!("Window resolution: {} x {}", app_state.width, app_state.height);

    // Skia and surfaces
    let mut skia = Skia::new();
    let mut surface = skia.make_surface(app_state.width * app_state.dpi as i32, app_state.height * app_state.dpi as i32);

    // Loop
    while !rl.window_should_close() {
        unsafe { handle_input(&mut app_state); }

        // Render
        let canvas = surface.skia_surface.canvas();
        skia.set_matrix_camera(canvas, &app_state);
        for territory in &app_state.territories {
            territory.1.render_polygons(canvas);
        }
        skia.clear_matrix(canvas);
        unsafe { render(&mut rl, &thread, &mut skia, &mut surface, &app_state); }
    }
}

unsafe fn render(rl: &mut RaylibHandle, thread: &RaylibThread, skia: &mut Skia, surface: &mut MySurface, app_state: &AppState) {

    // FPS
    let fps = format!("FPS: {}", rl.get_fps());
    let canvas = surface.skia_surface.canvas();
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::StrokeAndFill);
    paint.set_argb(255, 0, 0, 0);
    skia.write_text(canvas, 20.0 * app_state.dpi, &paint, fps.as_str(), 0.0, 0.0, 0.0);

    // Flush all Skia ops
    unsafe { skia.flush(surface); }

    // Do raylib render phase
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);
    let source = raylib::ffi::Rectangle { x: 0.0, y: 0.0, width: (app_state.width as f32) * app_state.dpi, height: (app_state.height as f32) * app_state.dpi };
    let dest = raylib::ffi::Rectangle { x: 0.0, y: 0.0, width: (app_state.width as f32), height: app_state.height as f32 };
    DrawTexturePro(
        surface.texture.texture,
        source, dest,
        Vector2 { x: 0.0, y: 0.0 }, 0.0,
        raylib::ffi::Color { a: 255, r: 255, g: 255, b: 255 });
}

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
unsafe fn handle_input(app_state: &mut AppState) {

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
