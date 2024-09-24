mod app_state;
mod skia;
mod cbor;
mod input;
mod render;

mod model {
    pub mod city;
    pub mod location;
    pub mod territory;
    pub mod territory_polygon;
}

use crate::input::handle_input;
use crate::render::render;
use crate::skia::Skia;
use app_state::AppState;
use raylib::ffi::{GetCurrentMonitor, GetMonitorHeight, GetMonitorWidth};

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
        unsafe { render(&mut rl, &thread, &mut skia, &mut surface, &app_state); }
    }
}

