use crate::app_state::GFXState;
use crate::lib::skia::{FontFamily, Skia};
use crate::model::world_state::WorldState;
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};

pub fn army_placement(skia: &mut Skia, world_state: &WorldState, gfx: &GFXState, rr: Rect) {
    skia.set_matrix(gfx);

    // Positions
    let l = rr.left + 50.0;
    let r = rr.right - 50.0;
    let w = r - l;

    // Title
    let mut paint_title = Paint::default();
    paint_title.set_anti_alias(true);
    paint_title.set_style(PaintStyle::StrokeAndFill);
    paint_title.set_color(Color::YELLOW);
    skia.write_text_centre(30.0, &paint_title, "Regiment Placement", Point::new(l, rr.top), w, &FontFamily::EbGaramond);

    // Numer of armies remaining
    let mut paint_left = Paint::default();
    paint_left.set_anti_alias(true);
    paint_left.set_style(PaintStyle::StrokeAndFill);
    paint_left.set_color(Color::LIGHT_GRAY);
    let mut paint_right = Paint::default();
    paint_right.set_anti_alias(true);
    paint_right.set_style(PaintStyle::StrokeAndFill);
    paint_right.set_color(Color::WHITE);
    skia.write_text_centre(
        25.0,
        &paint_left,
        "Click on a city to assign a regiment to defend the city, or to later attack enemy cities.",
        Point::new(l, rr.top + 60.0),
        w,
        &FontFamily::EbGaramond,
    );
    let mut ss = String::from("");
    let player = &world_state.current_player.as_ref().unwrap();
    for _ in 0..player.borrow().armies_to_assign {
        ss += "⚔";
    }
    skia.write_text_centre(30.0, &paint_right, &ss, Point::new(l, rr.top + 125.0), w, &FontFamily::NotoSansSymbols);

    skia.get_canvas().restore();
}
