use skia_safe::{Color, Paint, PaintStyle, Point, Rect};
use crate::app_state::AppState;
use crate::lib::skia::Skia;

pub fn randomising(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    skia.set_matrix(app_state);

    let w = rr.right - rr.left;

    // Title
    let mut paint_title = Paint::default();
    paint_title.set_anti_alias(true);
    paint_title.set_style(PaintStyle::StrokeAndFill);
    paint_title.set_color(Color::YELLOW);
    skia.write_text_centre(30.0, &paint_title, "Assigning Cities", Point::new(app_state.half_width as f32, rr.top), w);
    
    skia.get_canvas().restore();
}
