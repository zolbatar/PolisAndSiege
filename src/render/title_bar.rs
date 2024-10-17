use crate::app_state::{AppState, GameMode};
use crate::lib::skia::{FontFamily, Skia};
use skia_safe::{Color, Paint, PaintStyle, Point};

pub fn render_title_bar(skia: &mut Skia, app_state: &mut AppState) {
    let world_state = &app_state.world_state;
    let player = world_state.current_player.as_ref();
    skia.set_matrix(&app_state.gfx);

    // Show faction name
    let mut paint_title = Paint::default();
    paint_title.set_anti_alias(true);
    paint_title.set_style(PaintStyle::StrokeAndFill);
    paint_title.set_color(Color::YELLOW);

    // Title

    // Mode
    let phase = match world_state.mode {
        GameMode::Randomising => "Assigning Cities",
        GameMode::ArmyPlacement => "Initial Army Placement",
        GameMode::Game => {
            if player.is_none() {
                "No turn"
            } else if player.unwrap().borrow().is_human() {
                "Player Turn"
            } else {
                "Enemy Turn"
            }
        }
        GameMode::End => "Game over",
    };
    skia.write_text_centre(
        30.0,
        &paint_title,
        phase,
        Point::new(0.0, 0.0),
        app_state.gfx.width as f32,
        &FontFamily::EbGaramond,
    );
    if player.is_some() {
        paint_title.set_color(player.unwrap().borrow().colours[0]);
        skia.write_text(
            20.0,
            &paint_title,
            &player.unwrap().borrow().name,
            Point::new(160.0, 0.0),
            app_state.gfx.width as f32,
            &FontFamily::EbGaramond,
        );

        // City/territory count
        {
            let player = player.unwrap().borrow();
            skia.write_text_right(
                20.0,
                &paint_title,
                &format!("Score: {} Cities: {} of {}", player.score, player.cities.len(), world_state.cities.len(),),
                Point::new(0.0, 0.0),
                app_state.gfx.width as f32 - 160.0,
                &FontFamily::EbGaramond,
            );
        }
    }

    skia.get_canvas().restore();
}
