use std::time::Instant;
use rand::{thread_rng, Rng};
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};
use crate::app_state::{AppState, GameMode};
use crate::lib::skia::Skia;
use crate::model::city::Owner;

pub fn randomising(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    skia.set_matrix(&app_state.gfx);

    let w = rr.right - rr.left;

    // Title
    let mut paint_title = Paint::default();
    paint_title.set_anti_alias(true);
    paint_title.set_style(PaintStyle::StrokeAndFill);
    paint_title.set_color(Color::YELLOW);
    skia.write_text_centre(30.0, &paint_title, "Assigning Cities", Point::new(app_state.gfx.half_width as f32, rr.top), w);

    // Do we need to assign a new one?
    let diff = Instant::now() - app_state.last_selection;
    if diff.as_millis() > 1000 {
        app_state.last_selection = Instant::now();

        // Take top item
        if app_state.items.cities_remaining_to_assign.is_empty() {
            app_state.mode = GameMode::Game;
        } else {
            let next_city = app_state.items.cities_remaining_to_assign.pop().unwrap();
            let mut rng = thread_rng();
            let random_player = rng.gen_range(0..app_state.num_of_players - 1);
            match random_player {
                0 => next_city.lock().unwrap().owner = Owner::Player,
                1 => next_city.lock().unwrap().owner = Owner::Enemy1,
                2 => next_city.lock().unwrap().owner = Owner::Enemy2,
                3 => next_city.lock().unwrap().owner = Owner::Enemy3,
                4 => next_city.lock().unwrap().owner = Owner::Enemy4,
                _ => next_city.lock().unwrap().owner = Owner::None,
            }
        }
    }

    skia.get_canvas().restore();
}
