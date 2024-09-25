use crate::app_state::AppState;
use crate::skia::{MySurface, Skia};
use raylib::color::Color;
use raylib::ffi::{DrawTexturePro, Vector2};
use raylib::{RaylibHandle, RaylibThread};
use raylib::prelude::RaylibDraw;
use skia_safe::{Paint, PaintStyle};

pub unsafe fn render(rl: &mut RaylibHandle, thread: &RaylibThread, skia: &mut Skia, surface: &mut MySurface, app_state: &AppState) {

    // Render
    let canvas = surface.skia_surface.canvas();
    skia.set_matrix_camera(canvas, &app_state);
    for territory in &app_state.territories {
        territory.1.render_polygons(canvas);
        for city in &territory.1.cities {
            city.render(canvas, skia);
        }
    }
    skia.clear_matrix(canvas);

    // FPS
    let fps = format!("FPS: {}", rl.get_fps());
    let canvas = surface.skia_surface.canvas();
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::StrokeAndFill);
    paint.set_argb(255, 0, 0, 0);
    skia.write_text(canvas, 20.0 * app_state.dpi, &paint, fps.as_str(), 0.0, 0.0, 0.0);

    // Flush all Skia ops
    unsafe { skia.flush(surface); }

    // Do raylib render phase
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);
    let source = raylib::ffi::Rectangle { x: 0.0, y: 0.0, width: (app_state.width as f32) * app_state.dpi, height: (app_state.height as f32) * app_state.dpi };
    let dest = raylib::ffi::Rectangle { x: 0.0, y: 0.0, width: (app_state.width as f32), height: app_state.height as f32 };
    DrawTexturePro(
        surface.texture.texture,
        source, dest,
        Vector2 { x: 0.0, y: 0.0 }, 0.0,
        raylib::ffi::Color { a: 255, r: 255, g: 255, b: 255 });
}
