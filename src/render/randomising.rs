use std::time::Instant;
use rand::{thread_rng, Rng};
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};
use crate::app_state::{AppState, GameMode};
use crate::lib::skia::Skia;
use crate::model::city::Owner;

pub fn randomising(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    skia.set_matrix(&app_state.gfx);

    // Positions
    let offset_text = 125.0f32;
    let text_x = app_state.gfx.half_width as f32 - offset_text;
    let w = rr.right - rr.left;
    let text_w = w - offset_text;

    // Title
    let mut paint_title = Paint::default();
    paint_title.set_anti_alias(true);
    paint_title.set_style(PaintStyle::StrokeAndFill);
    paint_title.set_color(Color::YELLOW);
    skia.write_text_centre(30.0, &paint_title, "Assigning Cities", Point::new(app_state.gfx.half_width as f32, rr.top), w);

    // Do we need to assign a new one?
    let diff = Instant::now() - app_state.selection.last_selection;
    if diff.as_millis() > 1000 {
        app_state.selection.last_selection = Instant::now();

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
            app_state.selection.last_city_selection = Some(next_city.clone());
        }
    }

    // Name and territory
    if app_state.selection.last_city_selection.is_some() {
        let last_city = app_state.selection.last_city_selection.clone().unwrap();
        let mut paint_left = Paint::default();
        paint_left.set_anti_alias(true);
        paint_left.set_style(PaintStyle::StrokeAndFill);
        paint_left.set_color(Color::GRAY);
        let mut paint_right = Paint::default();
        paint_right.set_anti_alias(true);
        paint_right.set_style(PaintStyle::StrokeAndFill);
        paint_right.set_color(Color::WHITE);
        skia.write_text_right(20.0, &paint_left, "Name:  ", Point::new(text_x, rr.top + 60.0), text_w);
        skia.write_text(20.0, &paint_right, &last_city.lock().unwrap().name, Point::new(text_x, rr.top() + 60.0), text_w);
        skia.write_text_right(20.0, &paint_left, "Territory:  ", Point::new(text_x, rr.top + 85.0), text_w);
        skia.write_text(20.0, &paint_right, &last_city.lock().unwrap().territory.lock().unwrap().name, Point::new(text_x, rr.top + 85.0), text_w);
    }

    skia.get_canvas().restore();
}
