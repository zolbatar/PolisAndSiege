use skia_safe::{Paint, PaintStyle, Point, Vector};
use crate::app_state::AppState;
use crate::lib::skia::{Skia};

pub fn render(skia: &mut Skia, app_state: &mut AppState) {
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

    // FPS
    let fps = format!("Zoom: {}, Position: {}/{}", app_state.zoom, app_state.target.x, app_state.target.y);
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(skia_safe::Color::WHITE);
    skia.write_text(20.0 * app_state.dpi, &paint, fps.as_str(), Point::new(0.0, 0.0), 0.0);

    // Flush all Skia ops
    unsafe { skia.flush(); }
}
