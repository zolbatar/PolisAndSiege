use crate::app_state::{AppState};
use crate::lib::skia::{FontFamily, Skia};
use crate::model::city::City;
use crate::model::player::Player;
use crate::model::territory::Territory;
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};
use specs::WorldExt;
use std::time::Instant;
use crate::{next_turn};

fn assign(app_state: &mut AppState) {
    let next_city = app_state.items.cities_remaining_to_assign.pop().unwrap();
    let mut player = app_state.world.write_storage::<Player>();
    next_city.lock().unwrap().owner = Some(app_state.current_player);
    player.get_mut(app_state.current_player).unwrap().cities.push(next_city.clone());
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

    // Do we need to assign a new one?
    let diff = Instant::now() - app_state.selection.last_selection;
    if diff.as_millis() > app_state.selection.assign_speed {
        app_state.selection.last_selection = Instant::now();

        // Take top item
        if !app_state.items.cities_remaining_to_assign.is_empty() {
            assign(app_state);
            next_turn(app_state);
        }
    }

    // Name and territory
    if let Some(city_state) = &app_state.selection.last_army_city_selection {
        let cities = app_state.world.read_storage::<City>();
        let last_city = cities.get(city_state.lock().unwrap().city).unwrap();
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
            &last_city.name,
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
            &app_state.world.read_storage::<Territory>().get(last_city.territory).unwrap().name,
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
        let owner = city_state.lock().unwrap().owner.unwrap();
        let owner_string = app_state.world.read_storage::<Player>().get(owner).unwrap().name.clone();
        skia.write_text(
            20.0,
            &paint_right,
            &owner_string,
            Point::new(text_x, rr.top + 110.0),
            text_w,
            &FontFamily::EbGaramond,
        );
    }

    skia.get_canvas().restore();
}
