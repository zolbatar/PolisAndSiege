mod app_state;
mod skia;
mod cbor;
mod model {
    pub mod city;
    pub mod location;
    pub mod territory;
    pub mod territory_polygon;
}

use app_state::AppState;
use crate::cbor::Cbor;

fn main() {
    let (rl, thread) = raylib::init()
        .size(1600, 950)
        .title("Simulation")
        //        .undecorated()
        .build();

    // Load CBOR data
    let cbor = Cbor::new();

    // Create an AppState instance using the new method
    let mut state = AppState::new(rl, thread, cbor);

    // Loop
    while !state.rl.window_should_close() {
        let canvas = state.surface.skia_surface.canvas();
        state.skia.set_matrix(canvas, state.dpi);
        state.skia.test(canvas, state.width, state.height);
        state.skia.clear_matrix(canvas);
        unsafe { state.render(); }
    }
}
