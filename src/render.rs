use crate::app_state::AppState;
use crate::lib::skia::{Skia};

pub unsafe fn render(skia: &mut Skia, app_state: &mut AppState) {
    skia.reset_context();
    skia.set_matrix_camera(app_state);

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
/*    let fps = format!("FPS: {}", rl.get_fps());
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(skia_safe::Color::WHITE);
    skia.write_text(20.0 * app_state.dpi, &paint, fps.as_str(), Point::new(0.0, 0.0), 0.0);*/

    // Flush all Skia ops
    unsafe { skia.flush(); }
}
