use crate::app_state::{AppState, NOISE_MIX};
use crate::lib::skia::Skia;
use skia_safe::paint::Style;
use skia_safe::{Paint, Rect, Vector};

const WIDTH: f32 = 275.0;
const HEIGHT: f32 = 200.0;

pub fn render_lower_panel(skia: &mut Skia, app_state: &mut AppState) -> Rect {
    // Bottom section for drawing stuff
    let l = app_state.gfx.half_width as f32 - WIDTH;
    let r = app_state.gfx.half_width as f32 + WIDTH;
    let t = app_state.gfx.height as f32 - HEIGHT - 32.0;
    let b = t + HEIGHT;
    let rr = Rect::new(l, t, r, b);

    // Outer shape
    skia.set_matrix(&app_state.gfx);
    let mut paint_background = Paint::default();
    paint_background.set_anti_alias(true);
    paint_background.set_style(Style::Fill);
    paint_background.set_shader(skia.create_noise_shader(skia.colour_background, NOISE_MIX));
    skia.get_canvas().draw_round_rect(
        Rect::from_xywh(rr.left - 8.0, rr.top - 8.0, rr.right - rr.left + 16.0, rr.bottom - rr.top + 16.0),
        32.0,
        32.0,
        &paint_background,
    );
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

    rr
}
