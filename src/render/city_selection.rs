use crate::app_state::AppState;
use crate::lib::skia::Skia;
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};

pub fn city_selection(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    skia.set_matrix(&app_state.gfx);

    // City
    let city = app_state.items.cities.first().unwrap().lock().unwrap();
    let city_name = &city.name;
    let territory_name = &city.territory.lock().unwrap().name;

    // Positions
    let offset_text = 125.0f32;
    let text_x = app_state.gfx.half_width as f32 - offset_text;
    let w = rr.right - rr.left;
    let text_w = w - offset_text;

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
    skia.write_text_centre(
        30.0,
        &paint_title,
        "City Selection",
        Point::new(app_state.gfx.half_width as f32, rr.top),
        w,
    );

    // Name and territory
    let mut paint_left = Paint::default();
    paint_left.set_anti_alias(true);
    paint_left.set_style(PaintStyle::StrokeAndFill);
    paint_left.set_color(Color::GRAY);
    let mut paint_right = Paint::default();
    paint_right.set_anti_alias(true);
    paint_right.set_style(PaintStyle::StrokeAndFill);
    paint_right.set_color(Color::WHITE);
    skia.write_text_right(20.0, &paint_left, "Name:  ", Point::new(text_x, rr.top + 60.0), text_w);
    skia.write_text(20.0, &paint_right, city_name, Point::new(text_x, rr.top() + 60.0), text_w);
    skia.write_text_right(20.0, &paint_left, "Territory:  ", Point::new(text_x, rr.top + 85.0), text_w);
    skia.write_text(20.0, &paint_right, territory_name, Point::new(text_x, rr.top + 85.0), text_w);

    // Sections
    skia.get_canvas().restore();
}
