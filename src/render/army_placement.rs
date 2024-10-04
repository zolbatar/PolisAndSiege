use crate::app_state::AppState;
use crate::lib::skia::Skia;
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};

pub fn army_placement(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
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
    skia.write_text_centre(
        30.0,
        &paint_title,
        "Army Placement",
        Point::new(app_state.gfx.half_width as f32, rr.top),
        w,
    );

    skia.get_canvas().restore();
}
