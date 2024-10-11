use crate::app_state::{AppState, GameMode};
use crate::lib::skia::{FontFamily, Skia};
use crate::model::city::CCity;
use crate::model::player::CPlayer;
use crate::model::territory::CTerritory;
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};
use specs::WorldExt;
use std::time::Instant;
use crate::next_turn;

fn assign(app_state: &mut AppState) {
    let mut cities = app_state.world.write_storage::<CCity>();
    let next_player = *app_state.res.player_lookup.get(&app_state.selection.last_player).unwrap();
    let next_city = app_state.items.cities_remaining_to_assign.pop().unwrap();
    let mut player = app_state.world.write_storage::<CPlayer>();
    player.get_mut(next_player).unwrap().cities.push(next_city);

    //    player.get_mut(next_player).unwrap().score += next_city_obj.size as i32;
    cities.get_mut(next_city).unwrap().owner = Some(next_player);
    app_state.selection.last_player += 1;
    if app_state.selection.last_player >= app_state.num_of_players {
        app_state.selection.last_player = 0;
    }
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
    if app_state.selection.assign_speed == 0 {
        loop {
            if diff.as_millis() > app_state.selection.assign_speed {
                app_state.selection.last_selection = Instant::now();

                // Take top item
                if app_state.items.cities_remaining_to_assign.is_empty() {
                    app_state.selection.last_city_selection = None;
                    app_state.mode = GameMode::ArmyPlacement;
                    app_state.current_turn = app_state.actual_human;
                    return;
                } else {
                    assign(app_state);
                }
            }
        }
    } else if diff.as_millis() > app_state.selection.assign_speed {
        app_state.selection.last_selection = Instant::now();

        // Take top item
        if app_state.items.cities_remaining_to_assign.is_empty() {
            app_state.selection.last_city_selection = None;
            app_state.mode = GameMode::ArmyPlacement;
            app_state.current_turn = app_state.actual_human;
            return;
        } else {
            assign(app_state);
            next_turn(app_state);
        }
    }

    // Name and territory
    if let Some(city) = app_state.selection.last_army_city_selection {
        let cities = app_state.world.read_storage::<CCity>();
        let last_city = cities.get(city).unwrap();
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
            &app_state.world.read_storage::<CTerritory>().get(last_city.territory).unwrap().name,
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
        let owner = last_city.owner.clone().unwrap();
        let owner_string = app_state.world.read_storage::<CPlayer>().get(owner).unwrap().name.clone();
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
