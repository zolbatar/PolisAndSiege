use std::collections::{BTreeMap, HashMap};
use crate::app_state::AppState;
use crate::lib::skia::Skia;
use skia_safe::paint::Style;
use skia_safe::{Color, Paint, Point, RRect, Rect};

pub fn region_summary(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    skia.set_matrix(&app_state.gfx);

    // Paints
    let mut paint_white = Paint::default();
    paint_white.set_anti_alias(true);
    paint_white.set_style(Style::StrokeAndFill);
    paint_white.set_color(Color::WHITE);
    let mut paint_line = Paint::default();
    paint_line.set_anti_alias(true);
    paint_line.set_style(Style::Fill);
    paint_line.set_argb(255, 80, 80, 80);

    for (index, (fst, snd)) in app_state.items.territories.iter().enumerate() {
        let y = rr.top + 25.0 * index as f32;

        let mut paint_territory = Paint::default();
        paint_territory.set_color(snd.lock().unwrap().colour);
        paint_territory.set_anti_alias(true);
        paint_territory.set_style(Style::Fill);
        skia.get_canvas().draw_circle(Point::new(rr.left + 52.0, y + 13.0), 10.0, &paint_territory);

        // Name
        skia.write_text(20.0, &paint_white, fst, Point::new(rr.left() + 64.0, y), 0.0);
        if index % 2 == 0 {
            skia.get_canvas().draw_rrect(RRect::new_rect_xy(skia_safe::Rect::from_xywh(rr.left + 40.0, y + 26.0, rr.right - rr.left - 80.0, 24.0), 5.0, 5.0), &paint_line);
        }

        // Bonus?
        skia.write_text(20.0, &paint_white, "No bonus", Point::new(rr.right - 128.0, y), 0.0);

        // Work out proportions of ownership
        let mut map = HashMap::new();
        for city in &snd.lock().unwrap().cities {
            let owner = &city.lock().unwrap().owner;
            map.entry(owner.clone()).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut prop = BTreeMap::new();
        let total = snd.lock().unwrap().cities.len() as f32;
        for entry in map {
            prop.insert(entry.0, entry.1 as f32 / total * 64.0);
        }

        // Now draw bars for ownership
        let bar_start = 335.0;
        let bar_y_offer = 14.0;
        let mut xx = rr.left + bar_start;
        let yy = y + bar_y_offer;
        for entry in prop {
            let mut paint_player = Paint::default();
            paint_player.set_anti_alias(true);
            paint_player.set_style(Style::Stroke);
            paint_player.set_stroke_width(5.0);
            paint_player.set_color(app_state.res.player_colours.get(&entry.0).unwrap()[0]);
            paint_player.set_alpha(160);
            skia.get_canvas().draw_line(Point::new(xx, yy), Point::new(xx + entry.1, yy), &paint_player);
            xx += entry.1;
        }
        let mut paint_border = Paint::default();
        paint_border.set_anti_alias(true);
        paint_border.set_style(Style::Stroke);
        paint_border.set_stroke_width(1.0);
        paint_border.set_argb(255, 180, 180, 180);
        skia.get_canvas().draw_rrect(skia_safe::rrect::RRect::new_rect_xy(skia_safe::Rect::from_xywh(rr.left + bar_start, yy - 2.5, 64.0, 5.0), 2.0, 2.0), &paint_border);
    }

    skia.get_canvas().restore();
}

