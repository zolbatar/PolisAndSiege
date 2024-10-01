use crate::app_state::AppState;
use crate::lib::skia::Skia;
use skia_safe::{Paint, PaintStyle, RRect, Rect};

pub fn render_cityselection(skia: &mut Skia, app_state: &mut AppState) {
    let w = 200.0f32;
    let h = 200.0f32;
    skia.set_matrix(app_state);
    let rrect = RRect::new_rect_xy(Rect::from_xywh(app_state.half_width as f32 - w / 2.0, app_state.half_height as f32 - h / 2.0, w, h), 5.0, 5.0);
    let rrect_inner = RRect::new_rect_xy(Rect::from_xywh(app_state.half_width as f32 - w / 2.0 + 2.0, app_state.half_height as f32 - h / 2.0 + 2.0, w - 4.0, h - 4.0), 3.0, 3.0);

    // Paint for popup
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

    //    skia.get_canvas().draw_rrect(rrect, &paint_shadow);
    skia.get_canvas().draw_rrect(rrect, &paint);
    skia.get_canvas().draw_rrect(rrect, &paint_outline);
    skia.get_canvas().draw_rrect(rrect_inner, &paint_outline);

    skia.get_canvas().restore();
}

