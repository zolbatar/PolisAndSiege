use crate::app_state::AppState;
use crate::lib::skia::{FontFamily, Skia};
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};

pub fn assign(app_state: &mut AppState) {
    let world_state = &app_state.world_state;
    let next_city = app_state.world_fixed.city_states_to_assign.pop().unwrap();
    next_city.lock().unwrap().owner = world_state.current_player.clone();
    world_state.current_player.as_ref().unwrap().lock().unwrap().cities.push(next_city.clone());
    app_state.selection.last_army_city_selection = Some(next_city);
}

pub fn randomising(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    skia.set_matrix(&app_state.gfx);

    // Positions
    let l = rr.left + 50.0;
    let r = rr.right - 50.0;
    let label_width = 100.0f32;
    let w = r - l;
    let text_x = l + label_width + 10.0;
    let text_w = w - label_width - 10.0;

    // Title
    let mut paint_title = Paint::default();
    paint_title.set_anti_alias(true);
    paint_title.set_style(PaintStyle::StrokeAndFill);
    paint_title.set_color(Color::YELLOW);
    skia.write_text_centre(30.0, &paint_title, "Assigning Cities", Point::new(l, rr.top), w, &FontFamily::EbGaramond);

    // Name and territory
    if let Some(city_state) = &app_state.selection.last_army_city_selection {
        let last_city = &city_state.lock().unwrap().city;
        let mut paint_left = Paint::default();
        paint_left.set_anti_alias(true);
        paint_left.set_style(PaintStyle::StrokeAndFill);
        paint_left.set_color(Color::LIGHT_GRAY);
        let mut paint_right = Paint::default();
        paint_right.set_anti_alias(true);
        paint_right.set_style(PaintStyle::StrokeAndFill);
        paint_right.set_color(Color::WHITE);

        // Name
        skia.write_text_right(
            20.0,
            &paint_left,
            "Name:  ",
            Point::new(l, rr.top + 60.0),
            label_width,
            &FontFamily::EbGaramond,
        );
        skia.write_text(
            20.0,
            &paint_right,
            &last_city.lock().unwrap().name,
            Point::new(text_x, rr.top() + 60.0),
            text_w,
            &FontFamily::EbGaramond,
        );

        // Territory
        skia.write_text_right(
            20.0,
            &paint_left,
            "Territory:  ",
            Point::new(l, rr.top + 85.0),
            label_width,
            &FontFamily::EbGaramond,
        );
        skia.write_text(
            20.0,
            &paint_right,
            &last_city.lock().unwrap().territory.lock().unwrap().name,
            Point::new(text_x, rr.top + 85.0),
            text_w,
            &FontFamily::EbGaramond,
        );

        // Owner
        skia.write_text_right(
            20.0,
            &paint_left,
            "Owner:  ",
            Point::new(l, rr.top + 110.0),
            label_width,
            &FontFamily::EbGaramond,
        );
        skia.write_text(
            20.0,
            &paint_right,
            &last_city.lock().unwrap().name,
            Point::new(text_x, rr.top + 110.0),
            text_w,
            &FontFamily::EbGaramond,
        );
    }

    skia.get_canvas().restore();
}
