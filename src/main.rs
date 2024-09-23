mod app_state;
mod skia;

use app_state::AppState;

use raylib::prelude::*;
use skia_safe::{Paint, PaintStyle};
use skia_safe::Point;

fn main() {
    println!("Hello, world!");

    let (rl, thread) = raylib::init()
        .size(1600, 950)
        .title("Simulation")
        .undecorated()
        .build();

    // Create an AppState instance using the new method
    let mut state = AppState::new(rl, thread);
    state.skia.init(state.width, state.height);

    while !state.rl.window_should_close() {
        let canvas = state.skia.get_canvas();
        let mut paint = Paint::default();
        paint.set_style(PaintStyle::Stroke);
        canvas.draw_line(Point { x: 0.0, y: 0.0 }, Point { x: 100.0, y: 100.0 }, &paint);

        let mut d = state.rl.begin_drawing(&state.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
