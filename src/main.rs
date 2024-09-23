mod app_state;
mod skia;

use app_state::AppState;
use rand::Rng;
use skia_safe::{Paint, PaintStyle};
use skia_safe::Point;

fn main() {
    println!("Hello, world!");

    let (rl, thread) = raylib::init()
        .size(1600, 950)
        .title("Simulation")
        //        .undecorated()
        .build();

    // Create an AppState instance using the new method
    let mut state = AppState::new(rl, thread);

    let mut rng = rand::thread_rng();
    while !state.rl.window_should_close() {
        let canvas = state.surface.skia_surface.canvas();
        state.skia.set_matrix(canvas, state.dpi);
        let mut paint = Paint::default();
        paint.set_style(PaintStyle::Stroke);
        for _ in 1..=1000 {
            canvas.draw_line(Point { x: rng.gen_range(0..=state.width) as f32, y: rng.gen_range(0..=state.height) as f32 }, Point { x: rng.gen_range(0..=state.width) as f32, y: rng.gen_range(0..=state.height) as f32 }, &paint);
        }
        state.skia.clear_matrix(canvas);
        unsafe { state.render(); }
    }
}
