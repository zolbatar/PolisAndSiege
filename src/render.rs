use skia_safe::{BlurStyle, Color, MaskFilter, Paint, PaintStyle, Point, RRect, Rect, Vector};
use skia_safe::paint::Style;
use crate::app_state::AppState;
use crate::lib::skia::{Skia};
use crate::model::city::Owner;

fn render_region_summary(skia: &mut Skia, app_state: &mut AppState) {
    skia.set_matrix(app_state);

    // Territory paint
    let mut paint_territories = Paint::default();
    paint_territories.set_anti_alias(true);
    paint_territories.set_style(Style::Fill);
    //    paint_territories.set_color(skia.colour_background);
    paint_territories.set_shader(skia.create_noise_shader(skia.colour_background, 0.05));
    let blur = MaskFilter::blur(BlurStyle::Normal, 1.0, None).expect("Blur mask filter failed");
    paint_territories.set_mask_filter(blur);

    let territory_l = 525.0f32;
    let territory_r = app_state.width as f32 - territory_l - 1.0;
    let territory_t = app_state.height as f32 - 200.0 - 32.0;

    // Outer shape
    skia.get_canvas().draw_round_rect(Rect::from_xywh(territory_l - 8.0, territory_t - 8.0,
                                                      territory_r - territory_l + 16.0, 200.0 + 16.0), 32.0, 32.0, &paint_territories);

    // Sides
    {
        let canvas = skia.get_canvas();
        canvas.save();

        canvas.reset_matrix();
        canvas.translate(Vector::new(territory_l * app_state.dpi, territory_t * app_state.dpi));
        canvas.scale((1.0, 1.13));
        app_state.side_path.render(canvas);

        canvas.reset_matrix();
        canvas.translate(Vector::new(territory_r * app_state.dpi, territory_t * app_state.dpi));
        canvas.scale((-1.0, 1.13));
        app_state.side_path.render(canvas);

        canvas.restore();
    }

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
        let y = territory_t + 25.0 * index as f32;

        let mut paint_territory = Paint::default();
        paint_territory.set_color(snd.colour);
        paint_territory.set_anti_alias(true);
        paint_territory.set_style(Style::Fill);
        skia.get_canvas().draw_circle(Point::new(territory_l + 52.0, y + 13.0), 10.0, &paint_territory);

        // Name
        skia.write_text(20.0, &paint_white, fst, Point::new(territory_l + 64.0, y), 0.0);
        if index % 2 == 0 {
            skia.get_canvas().draw_rrect(RRect::new_rect_xy(skia_safe::Rect::from_xywh(territory_l + 40.0, y + 26.0, territory_r - territory_l - 80.0, 24.0), 5.0, 5.0), &paint_line);
        }

        // Bonus?
        skia.write_text(20.0, &paint_white, "No bonus", Point::new(territory_r - 128.0, y), 0.0);

        let mut player = 0;
        let mut enemy = 0;
        let mut none = 0;

        for city in &snd.cities {
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
        let xx = territory_l + bar_start;
        let yy = y + bar_y_offer;
        skia.get_canvas().draw_line(Point::new(xx, yy), Point::new(xx + player_segment, yy), &paint_player);
        skia.get_canvas().draw_line(Point::new(xx + player_segment, yy), Point::new(xx + player_segment + enemy_segment, yy), &paint_enemy);
        skia.get_canvas().draw_line(Point::new(xx + player_segment + enemy_segment, yy), Point::new(xx + player_segment + enemy_segment + none_segment, yy), &paint_none);
    }

    skia.get_canvas().restore();
}

pub fn render(skia: &mut Skia, app_state: &mut AppState) {
    skia.reset_context();

    let clip_rect = RRect::new_rect_xy(Rect::from_xywh(32.0, 32.0, app_state.width as f32 - 64.0, app_state.height as f32 - 64.0), 128.0, 128.0);
    {
        let canvas = skia.get_canvas();
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((app_state.dpi, app_state.dpi));

        // Clip the edges
        canvas.clip_rrect(clip_rect, None, true);

        canvas.translate((app_state.half_width, app_state.half_height));
        canvas.scale((app_state.zoom, app_state.zoom));
        canvas.translate((-app_state.target.x, -app_state.target.y));
    }

    // Territories
    for territory in &app_state.territories {
        territory.1.render_polygons(skia.get_canvas());
    }

    // Connections
    for connection in app_state.connections.iter_mut() {
        connection.render(skia.get_canvas());
    }

    // Cities
    for territory in &app_state.territories {
        for city in &territory.1.cities {
            city.lock().unwrap().render(skia, app_state);
        }
    }
    skia.clear_matrix();

    // Blur edges
    {
        skia.set_matrix(app_state);
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_shader(skia.create_noise_shader(skia.colour_background, 0.05));
        paint.set_stroke_width(30.0);
        let blur = MaskFilter::blur(BlurStyle::Normal, 5.0, None).expect("Blur mask filter failed");
        paint.set_mask_filter(blur);
        skia.get_canvas().draw_rrect(clip_rect, &paint);
        skia.get_canvas().restore();
    }

    // Pretty surround
    {
        let width_dpi = app_state.width as f32 * app_state.dpi;
        let height_dpi = app_state.height as f32 * app_state.dpi;
        let canvas = skia.get_canvas();

        // Top-left
        canvas.save();
        canvas.reset_matrix();
        canvas.translate(Vector::new(16.0, 16.0));
        app_state.corner_path.render(canvas); //, 8, 8, 1, 1);
        canvas.restore();

        // Bottom-left
        canvas.save();
        canvas.translate(Vector::new(16.0, height_dpi - 16.0));
        canvas.scale((1.0, -1.0));
        app_state.corner_path.render(canvas);
        canvas.restore();

        // Top-right
        canvas.save();
        canvas.translate(Vector::new(width_dpi - 16.0 - 1.0, 16.0));
        canvas.scale((-1.0, 1.0));
        app_state.corner_path.render(canvas);
        canvas.restore();

        // Bottom-right
        canvas.save();
        canvas.translate(Vector::new(width_dpi - 16.0 - 1.0, height_dpi - 16.0 - 1.0));
        canvas.scale((-1.0, -1.0));
        app_state.corner_path.render(canvas);
        canvas.restore();
    }

    render_region_summary(skia, app_state);

    // FPS
    let fps = format!("Zoom: {}, Position: {}/{}", app_state.zoom, app_state.target.x, app_state.target.y);
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(skia_safe::Color::WHITE);
//    skia.write_text(20.0 * app_state.dpi, &paint, fps.as_str(), Point::new(0.0, 0.0), 0.0);

    // Flush all Skia ops
    unsafe { skia.flush(); }
}
