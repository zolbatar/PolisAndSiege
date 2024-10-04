use crate::app_state::AppState;
use crate::lib::skia::{FontFamily, Skia};
use crate::model::city::Owner;
use skia_safe::{Color, Paint, PaintStyle, Point};

pub fn render_title_bar(skia: &mut Skia, app_state: &mut AppState) {
    skia.set_matrix(&app_state.gfx);

    // Show faction name
    let mut paint_title = Paint::default();
    paint_title.set_anti_alias(true);
    paint_title.set_style(PaintStyle::StrokeAndFill);
    paint_title.set_color(Color::WHITE);
    skia.write_text_centre(
        35.0,
        &paint_title,
        app_state.res.player_name.get(&Owner::Player).unwrap(),
        Point::new(0.0, 0.0),
        app_state.gfx.width as f32,
        &FontFamily::EbGaramond,
    );

    skia.get_canvas().restore();
}
