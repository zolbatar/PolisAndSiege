mod app_state;
use app_state::AppState;

use raylib::prelude::*;

fn main() {
    println!("Hello, world!");

    let (rl, thread) = raylib::init()
        .size(1600, 950)
        .title("Simulation")
        .undecorated()
        .build();

    // Create an AppState instance using the new method
    let mut state = AppState::new(rl, thread);

    while !state.rl.window_should_close() {
        let mut d = state.rl.begin_drawing(&state.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
