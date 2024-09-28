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

use crate::input::handle_input;
use crate::render::render;
use crate::lib::skia::Skia;
use app_state::AppState;
use crate::lib::cbor;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1600, 950)
        .title("Simulation")
        //        .undecorated()
        .build();

    // Create an AppState instance using the new method
    let mut app_state = AppState::new(&rl);

    // Load CBOR data
    let territories = cbor::import(&mut app_state);
    app_state.territories = territories;

    // Skia and surfaces
    let mut skia = Skia::new();
    let mut surface = skia.make_surface(app_state.width * app_state.dpi as i32, app_state.height * app_state.dpi as i32);
    unsafe { skia.flush(&mut surface); }

    // Loop
    while !rl.window_should_close() {
        unsafe { handle_input(&mut app_state); }
        unsafe { render(&mut rl, &thread, &mut skia, &mut surface, &mut app_state); }
    }
}

