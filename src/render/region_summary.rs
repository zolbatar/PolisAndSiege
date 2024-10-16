use crate::app_state::AppState;
use crate::lib::skia::{FontFamily, Skia};
use skia_safe::paint::Style;
use skia_safe::{Color, Paint, Point, RRect, Rect};

pub fn region_summary(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    let world_fixed = &app_state.world_fixed;
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
    let mut paint_border = Paint::default();
    paint_border.set_anti_alias(true);
    paint_border.set_style(Style::Stroke);
    paint_border.set_stroke_width(1.0);
    paint_border.set_color(skia.colour_outline);

    {
        for (index, territory) in world_fixed.territories.iter().enumerate() {
            let y = rr.top + 25.0 * index as f32;

            let mut paint_territory = Paint::default();
            paint_territory.set_color(territory.1.lock().unwrap().colour);
            paint_territory.set_anti_alias(true);
            paint_territory.set_style(Style::Fill);
            skia.get_canvas().draw_circle(Point::new(rr.left + 52.0, y + 13.0), 7.0, &paint_territory);
            skia.get_canvas().draw_circle(Point::new(rr.left + 52.0, y + 13.0), 9.0, &paint_border);

            // Name
            skia.write_text(
                20.0,
                &paint_white,
                &territory.1.lock().unwrap().name,
                Point::new(rr.left() + 66.0, y),
                0.0,
                &FontFamily::EbGaramond,
            );

            if index % 2 == 0 {
                skia.get_canvas().draw_rrect(
                    RRect::new_rect_xy(
                        Rect::from_xywh(rr.left + 40.0, y + 26.0, rr.right - rr.left - 80.0, 24.0),
                        5.0,
                        5.0,
                    ),
                    &paint_line,
                );
            }

            // Bonus?
            let msg = "No bonus".to_owned();
            skia.write_text(20.0, &paint_white, &msg, Point::new(rr.right - 128.0, y), 0.0, &FontFamily::EbGaramond);

            // Work out proportions of ownership
            /*            let mut map = HashMap::new();
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
            let mut paint_player = Paint::default();
            paint_player.set_anti_alias(true);
            paint_player.set_style(Style::Stroke);
            paint_player.set_stroke_width(4.0);
            for entry in prop {
                paint_player.set_color(app_state.res.player_colours.get(&entry.0.unwrap()).unwrap()[0]);
                skia.get_canvas().draw_line(Point::new(xx, yy), Point::new(xx + entry.1, yy), &paint_player);
                xx += entry.1;
            }
            skia.get_canvas().draw_rrect(
                skia_safe::rrect::RRect::new_rect_xy(
                    Rect::from_xywh(rr.left + bar_start - 2.0, yy - 2.5 - 2.0, 64.0 + 4.0, 5.0 + 4.0),
                    3.0,
                    3.0,
                ),
                &paint_border,
            );*/
        }
    }

    skia.get_canvas().restore();
}
