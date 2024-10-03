use crate::app_state::{AppState, GameMode, NOISE_MIX};
use crate::lib::skia::Skia;
use crate::render::city_selection::city_selection;
use crate::render::region_summary::region_summary;
use crate::render::randomising::randomising;
use skia_safe::{BlurStyle, MaskFilter, Paint, PaintStyle, Point, RRect, Rect, Vector};
use skia_safe::paint::Style;

pub fn main(skia: &mut Skia, app_state: &mut AppState) {
    skia.reset_context();

    let clip_rect = RRect::new_rect_xy(Rect::from_xywh(32.0, 32.0, app_state.gfx.width as f32 - 64.0, app_state.gfx.height as f32 - 64.0), 128.0, 128.0);
    {
        let canvas = skia.get_canvas();
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((app_state.gfx.dpi, app_state.gfx.dpi));

        // Clip the edges
        canvas.clip_rrect(clip_rect, None, true);

        canvas.translate((app_state.gfx.half_width, app_state.gfx.half_height));
        canvas.scale((app_state.zoom, app_state.zoom));
        canvas.translate((-app_state.target.x, -app_state.target.y));
    }

    // Territories
    for territory in &app_state.items.territories {
        territory.1.lock().unwrap().render_polygons(skia.get_canvas());
    }

    // Connections
    for connection in app_state.items.connections.iter_mut() {
        connection.render(app_state.phase, skia.get_canvas());
    }

    // Cities
    for territory in &app_state.items.territories {
        for city in &territory.1.lock().unwrap().cities {
            city.lock().unwrap().render(skia, app_state);
        }
    }
    skia.clear_matrix();

    // Blur edges
    {
        skia.set_matrix(&app_state.gfx);
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_shader(skia.create_noise_shader(skia.colour_background, NOISE_MIX));
        paint.set_stroke_width(30.0);
        let blur = MaskFilter::blur(BlurStyle::Normal, 5.0, None).expect("Blur mask filter failed");
        paint.set_mask_filter(blur);
        skia.get_canvas().draw_rrect(clip_rect, &paint);
        skia.get_canvas().restore();
    }

    // Pretty surround
    {
        let width_dpi = app_state.gfx.width as f32 * app_state.gfx.dpi;
        let height_dpi = app_state.gfx.height as f32 * app_state.gfx.dpi;
        let canvas = skia.get_canvas();

        // Top-left
        canvas.save();
        canvas.reset_matrix();
        canvas.translate(Vector::new(16.0, 16.0));
        app_state.res.corner_path.render(canvas); //, 8, 8, 1, 1);
        canvas.restore();

        // Bottom-left
        canvas.save();
        canvas.translate(Vector::new(16.0, height_dpi - 16.0));
        canvas.scale((1.0, -1.0));
        app_state.res.corner_path.render(canvas);
        canvas.restore();

        // Top-right
        canvas.save();
        canvas.translate(Vector::new(width_dpi - 16.0 - 1.0, 16.0));
        canvas.scale((-1.0, 1.0));
        app_state.res.corner_path.render(canvas);
        canvas.restore();

        // Bottom-right
        canvas.save();
        canvas.translate(Vector::new(width_dpi - 16.0 - 1.0, height_dpi - 16.0 - 1.0));
        canvas.scale((-1.0, -1.0));
        app_state.res.corner_path.render(canvas);
        canvas.restore();
    }

    // Bottom section for drawing stuff
    let l = 525.0f32;
    let r = app_state.gfx.width as f32 - l - 1.0;
    let t = app_state.gfx.height as f32 - 200.0 - 32.0;
    let b = t + 200.0;
    let rr = Rect::new(l, t, r, b);

    // Outer shape
    skia.set_matrix(&app_state.gfx);
    let mut paint_background = Paint::default();
    paint_background.set_anti_alias(true);
    paint_background.set_style(Style::Fill);
    paint_background.set_shader(skia.create_noise_shader(skia.colour_background, NOISE_MIX));
    //    let blur = MaskFilter::blur(BlurStyle::Normal, 1.0, None).expect("Blur mask filter failed");
    //    paint_background.set_mask_filter(blur);
    skia.get_canvas().draw_round_rect(Rect::from_xywh(rr.left - 8.0, rr.top - 8.0,
                                                      rr.right - rr.left + 16.0, rr.bottom - rr.top + 16.0), 32.0, 32.0, &paint_background);
    skia.get_canvas().restore();

    // Sides
    {
        let canvas = skia.get_canvas();
        canvas.save();

        canvas.reset_matrix();
        canvas.translate(Vector::new(rr.left * app_state.gfx.dpi, rr.top * app_state.gfx.dpi));
        canvas.scale((1.0, 1.13));
        app_state.res.side_path.render(canvas);

        canvas.reset_matrix();
        canvas.translate(Vector::new(rr.right * app_state.gfx.dpi, rr.top * app_state.gfx.dpi));
        canvas.scale((-1.0, 1.13));
        app_state.res.side_path.render(canvas);

        canvas.restore();
    }

    // Now, render based on mode
    match app_state.mode {
        GameMode::Randomising => {
            randomising(skia, app_state, rr);
        }
        GameMode::CitySelection => {
            city_selection(skia, app_state, rr);
        }
        GameMode::Game => {
            region_summary(skia, app_state, rr);
        }
    }

    // FPS
    let fps = format!("FPS: {:.0} Zoom: {}, Position: {}/{}", app_state.fps, app_state.zoom, app_state.target.x, app_state.target.y);
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(skia_safe::Color::WHITE);
    skia.write_text(20.0 * app_state.gfx.dpi, &paint, fps.as_str(), Point::new(0.0, 0.0), 0.0);

    // Flush all Skia ops
    unsafe { skia.flush(); }
}
