use crate::app_state::AppState;
use crate::lib::skia::{Skia};
use raylib::color::Color;
use raylib::ffi::{DrawTexturePro, Vector2};
use raylib::{RaylibHandle, RaylibThread};
use raylib::prelude::RaylibDraw;
use skia_safe::{Paint, PaintStyle, Point};

pub unsafe fn render(rl: &mut RaylibHandle, thread: &RaylibThread, skia: &mut Skia, app_state: &mut AppState) {
    skia.set_matrix_camera(&app_state);

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

    // FPS
    let fps = format!("FPS: {}", rl.get_fps());
    let canvas = skia.surface.skia_surface.canvas();
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(skia_safe::Color::WHITE);
    skia.write_text(20.0 * app_state.dpi, &paint, fps.as_str(), Point::new(0.0, 0.0), 0.0);

    // Flush all Skia ops
    unsafe { skia.flush(); }

    // Do raylib render phase
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);
    let source = raylib::ffi::Rectangle { x: 0.0, y: 0.0, width: (app_state.width as f32) * app_state.dpi, height: (app_state.height as f32) * app_state.dpi };
    let dest = raylib::ffi::Rectangle { x: 0.0, y: 0.0, width: (app_state.width as f32), height: app_state.height as f32 };
    DrawTexturePro(
        skia.surface.texture.texture,
        source, dest,
        Vector2 { x: 0.0, y: 0.0 }, 0.0,
        raylib::ffi::Color { a: 255, r: 255, g: 255, b: 255 });
}
