use crate::app_state::AppState;
use crate::lib::skia::Skia;
use crate::model::city::Owner;
use skia_safe::paint::Style;
use skia_safe::{Color, Paint, Point, RRect, Rect};

pub fn render_region_summary(skia: &mut Skia, app_state: &mut AppState, rr: Rect) {
    skia.set_matrix(app_state);

    // Paints
    let mut paint_white = Paint::default();
    paint_white.set_anti_alias(true);
    paint_white.set_style(Style::StrokeAndFill);
    paint_white.set_color(Color::WHITE);
    let mut paint_line = Paint::default();
    paint_line.set_anti_alias(true);
    paint_line.set_style(Style::Fill);
    paint_line.set_argb(255, 80, 80, 80);
    let mut paint_player = Paint::default();
    paint_player.set_anti_alias(true);
    paint_player.set_style(Style::Stroke);
    paint_player.set_stroke_width(5.0);
    paint_player.set_color(Color::BLUE);
    let mut paint_enemy = Paint::default();
    paint_enemy.set_anti_alias(true);
    paint_enemy.set_style(Style::Stroke);
    paint_enemy.set_stroke_width(5.0);
    paint_enemy.set_color(Color::RED);
    let mut paint_none = Paint::default();
    paint_none.set_anti_alias(true);
    paint_none.set_style(Style::Stroke);
    paint_none.set_stroke_width(5.0);
    paint_none.set_argb(255, 90, 90, 90);

    for (index, (fst, snd)) in app_state.territories.iter().enumerate() {
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

        let mut player = 0;
        let mut enemy = 0;
        let mut none = 0;

        for city in &snd.lock().unwrap().cities {
            match city.lock().unwrap().owner {
                Owner::None => {
                    none += 1;
                }
                Owner::Player => {
                    player += 1;
                }
                Owner::Enemy1 | Owner::Enemy2 | Owner::Enemy3 | Owner::Enemy4 => {
                    enemy += 1;
                }
            }
        }

        let total = player as f32 + enemy as f32 + none as f32;
        let player_segment = player as f32 / total * 64.0;
        let enemy_segment = enemy as f32 / total * 64.0;
        let none_segment = none as f32 / total * 64.0;

        // Now draw bars for ownership
        let bar_start = 335.0;
        let bar_y_offer = 14.0;
        let xx = rr.left + bar_start;
        let yy = y + bar_y_offer;
        skia.get_canvas().draw_line(Point::new(xx, yy), Point::new(xx + player_segment, yy), &paint_player);
        skia.get_canvas().draw_line(Point::new(xx + player_segment, yy), Point::new(xx + player_segment + enemy_segment, yy), &paint_enemy);
        skia.get_canvas().draw_line(Point::new(xx + player_segment + enemy_segment, yy), Point::new(xx + player_segment + enemy_segment + none_segment, yy), &paint_none);
    }

    skia.get_canvas().restore();
}

