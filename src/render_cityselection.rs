use crate::app_state::AppState;
use crate::lib::skia::Skia;
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};

pub fn render_cityselection(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    skia.set_matrix(app_state);

    let mut paint_shadow = Paint::default();
    paint_shadow.set_anti_alias(true);
    paint_shadow.set_style(PaintStyle::Fill);
    paint_shadow.set_image_filter(skia.drop_shadow.clone());
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(skia.colour_popup);
    let mut paint_outline = Paint::default();
    paint_outline.set_anti_alias(true);
    paint_outline.set_style(PaintStyle::Stroke);
    paint_outline.set_color(skia.colour_outline);
    paint_outline.set_stroke_width(1.0);

    // Title
    let mut paint_title = Paint::default();
    paint_title.set_anti_alias(true);
    paint_title.set_style(PaintStyle::StrokeAndFill);
    paint_title.set_color(Color::YELLOW);
    let w = rr.right - rr.left;
    skia.write_text_centre(30.0, &paint_title, "City Selection", Point::new(app_state.half_width as f32, rr.top), w);

    // Name and territory
    let mut paint_left = Paint::default();
    paint_left.set_anti_alias(true);
    paint_left.set_style(PaintStyle::StrokeAndFill);
    paint_left.set_color(Color::GRAY);
    let mut paint_right = Paint::default();
    paint_right.set_anti_alias(true);
    paint_right.set_style(PaintStyle::StrokeAndFill);
    paint_right.set_color(Color::WHITE);
    skia.write_text_right(20.0, &paint_left, "Name:  ", Point::new(app_state.half_width as f32, rr.top + 60.0), w);
    skia.write_text(20.0, &paint_right, "City", Point::new(app_state.half_width as f32, rr.top() + 60.0), w);
    skia.write_text_right(20.0, &paint_left, "Territory:  ", Point::new(app_state.half_width as f32, rr.top + 85.0), w);
    skia.write_text(20.0, &paint_right, "Asia", Point::new(app_state.half_width as f32, rr.top + 85.0), w);

    // Sections
    skia.get_canvas().restore();
}

