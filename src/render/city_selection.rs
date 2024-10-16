use crate::app_state::AppState;
use crate::lib::skia::{FontFamily, Skia};
use skia_safe::{Color, Paint, PaintStyle, Point, Rect, Vector};

pub fn city_selection(skia: &mut Skia, app_state: &AppState, rr: Rect) {
    skia.set_matrix(&app_state.gfx);

    // Positions
    let l = rr.left + 50.0;
    let r = rr.right - 50.0;
    let w = r - l;
    let t = rr.top;

    // City
    let city = app_state.selection.last_city_selection.clone().unwrap();
    let city_name = &city.borrow().name;
    let territory_name = city.borrow().territory.name.clone();

    // Positions
    let text_w = 80.0;
    let text_x = l + text_w + 10.0;

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
    skia.write_text_centre(30.0, &paint_title, "City Selected", Point::new(l, t), w, &FontFamily::EbGaramond);

    // Name and territory
    let mut paint_left = Paint::default();
    paint_left.set_anti_alias(true);
    paint_left.set_style(PaintStyle::StrokeAndFill);
    paint_left.set_color(Color::LIGHT_GRAY);
    let mut paint_right = Paint::default();
    paint_right.set_anti_alias(true);
    paint_right.set_style(PaintStyle::StrokeAndFill);
    paint_right.set_color(Color::WHITE);
    skia.write_text_right(20.0, &paint_left, "Name:  ", Point::new(l, t + 60.0), text_w, &FontFamily::EbGaramond);
    skia.write_text(20.0, &paint_right, &city_name, Point::new(text_x, t + 60.0), 0.0, &FontFamily::EbGaramond);
    skia.write_text_right(20.0, &paint_left, "Territory:  ", Point::new(l, t + 85.0), text_w, &FontFamily::EbGaramond);
    skia.write_text(20.0, &paint_right, &territory_name, Point::new(text_x, t + 85.0), 0.0, &FontFamily::EbGaramond);

    // Combat
    skia.button("Attack!", app_state, Vector::new(app_state.gfx.half_width as f32, t + 120.0));

    // Sections
    skia.get_canvas().restore();
}
