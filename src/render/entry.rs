use std::sync::Arc;
use crate::app_state::{AppState, GameMode};
use crate::lib::skia::Skia;
use crate::render::army_placement::army_placement;
use crate::render::city_selection::city_selection;
use crate::render::lower_panel::render_lower_panel;
use crate::render::randomising::randomising;
use crate::render::region_summary::region_summary;
use crate::render::surround::render_surround;
use crate::render::title_bar::render_title_bar;
use skia_safe::{Paint, PaintStyle, RRect, Rect};

pub fn main(skia: &mut Skia, app_state: &mut AppState) {
    skia.reset_context();

    let clip_rect = RRect::new_rect_xy(
        Rect::from_xywh(32.0, 32.0, app_state.gfx.width as f32 - 64.0, app_state.gfx.height as f32 - 64.0),
        128.0,
        128.0,
    );
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
        connection.render(skia, app_state.phase);
    }

    // Cities
    for territory in &app_state.items.territories {
        for city in &territory.1.lock().unwrap().cities {
            if let Some(selected_city) = app_state.selection.last_city_selection.clone() {
                let selected = Arc::ptr_eq(&selected_city, &city.clone());
                city.lock().unwrap().render(skia, app_state, selected);
            } else {
                city.lock().unwrap().render(skia, app_state, false);
            }
        }
    }
    skia.clear_matrix();

    // Elements
    render_surround(skia, app_state, clip_rect);
    let rr = render_lower_panel(skia, app_state);
    render_title_bar(skia, app_state);

    // Now, render based on mode
    match app_state.mode {
        GameMode::Randomising => {
            randomising(skia, app_state, rr);
        }
        GameMode::ArmyPlacement => {
            army_placement(skia, app_state, rr);
        }
        GameMode::Game => {
            if app_state.selection.last_city_selection.is_some() {
                city_selection(skia, app_state, rr);
            } else {
                region_summary(skia, app_state, rr);
            }
        }
    }

    // FPS
    let fps = format!(
        "FPS: {:.0} Zoom: {}, Position: {}/{}",
        app_state.fps, app_state.zoom, app_state.target.x, app_state.target.y
    );
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(skia_safe::Color::WHITE);
    //skia.write_text(20.0 * app_state.gfx.dpi, &paint, fps.as_str(), Point::new(0.0, 0.0), 0.0);

    // Flush all Skia ops
    unsafe {
        skia.flush();
    }
}
