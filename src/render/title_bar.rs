use crate::app_state::{AppState, GameMode};
use crate::lib::skia::{FontFamily, Skia};
use crate::model::player::Player;
use skia_safe::{Color, Paint, PaintStyle, Point};
use specs::WorldExt;

pub fn render_title_bar(skia: &mut Skia, app_state: &mut AppState) {
    let players = app_state.world.read_storage::<Player>();
    let player = players.get(app_state.current_player).unwrap();
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
            if app_state.current_player == app_state.actual_human {
                "Player Turn"
            } else {
                "Enemy Turn"
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
    paint_title.set_color(player.colours[0]);
    skia.write_text(
        20.0,
        &paint_title,
        &player.name.clone(),
        Point::new(160.0, 0.0),
        app_state.gfx.width as f32,
        &FontFamily::EbGaramond,
    );

    // City/territory count
    skia.write_text_right(
        20.0,
        &paint_title,
        &format!("Score: {} Cities: {} of {}", player.score, player.cities.len(), app_state.items.cities.len()),
        Point::new(0.0, 0.0),
        app_state.gfx.width as f32 - 160.0,
        &FontFamily::EbGaramond,
    );

    skia.get_canvas().restore();
}
