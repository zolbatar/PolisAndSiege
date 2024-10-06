use crate::app_state::{AppState, NOISE_MIX};
use crate::lib::skia::Skia;
use skia_safe::{BlurStyle, MaskFilter, Paint, PaintStyle, RRect, Vector};

pub fn render_surround(skia: &mut Skia, app_state: &mut AppState, clip_rect: RRect) {
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
        canvas.reset_matrix();
        canvas.translate(Vector::new(16.0, height_dpi - 16.0));
        canvas.scale((1.0, -1.0));
        app_state.res.corner_path.render(canvas);
        canvas.restore();

        // Top-right
        canvas.save();
        canvas.reset_matrix();
        canvas.translate(Vector::new(width_dpi - 16.0 - 1.0, 16.0));
        canvas.scale((-1.0, 1.0));
        app_state.res.corner_path.render(canvas);
        canvas.restore();

        // Bottom-right
        canvas.save();
        canvas.reset_matrix();
        canvas.translate(Vector::new(width_dpi - 16.0 - 1.0, height_dpi - 16.0 - 1.0));
        canvas.scale((-1.0, -1.0));
        app_state.res.corner_path.render(canvas);
        canvas.restore();
    }
}
