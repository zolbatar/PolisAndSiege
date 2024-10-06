use crate::app_state::{AppState, GameMode};
use crate::lib::skia::{FontFamily, Skia};
use crate::model::city::Owner;
use skia_safe::{Color, Paint, PaintStyle, Point};

pub fn render_title_bar(skia: &mut Skia, app_state: &mut AppState) {
    skia.set_matrix(&app_state.gfx);

    // Show faction name
    let mut paint_title = Paint::default();
    paint_title.set_anti_alias(true);
    paint_title.set_style(PaintStyle::StrokeAndFill);
    paint_title.set_color(Color::YELLOW);

    // Title

    // Mode
    let phase = match app_state.mode {
        GameMode::Randomising => "Assigning Cities",
        GameMode::ArmyPlacement => "Initial Army Placement",
        GameMode::Game => {
            match app_state.current_turn {
                Owner::Player => "Player Turn",
                _ => "Enemy Turn"
            }
        }
    };
    skia.write_text_centre(
        30.0,
        &paint_title,
        phase,
        Point::new(0.0, 0.0),
        app_state.gfx.width as f32,
        &FontFamily::EbGaramond,
    );
    paint_title.set_color(Color::WHITE);
    skia.write_text(
        25.0,
        &paint_title,
        app_state.res.player_name.get(&Owner::Player).unwrap(),
        Point::new(160.0, 0.0),
        app_state.gfx.width as f32,
        &FontFamily::EbGaramond,
    );

    let mut total_cities = 0;
    let mut cities = 0;
    for territory in &app_state.items.territories {
        for city in territory.1.lock().unwrap().cities.iter() {
            if city.lock().unwrap().owner == Owner::Player {
                cities += 1;
            }
            total_cities += 1;
        }
    }

    // City/territory count
    skia.write_text_right(
        25.0,
        &paint_title,
        &format!("Cities: {}/{}", cities, total_cities),
        Point::new(0.0, 0.0),
        app_state.gfx.width as f32 - 160.0,
        &FontFamily::EbGaramond,
    );

    skia.get_canvas().restore();
}
