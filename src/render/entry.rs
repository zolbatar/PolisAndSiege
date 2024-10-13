use crate::app_state::{AppState, GameMode};
use crate::lib::skia;
use crate::lib::skia::{FontFamily, Skia};
use crate::model::city::{City, MAXIMUM_LABEL_WIDTH, SIZE, SIZE_SELECTED};
use crate::model::connection::{Connection, LINE_WIDTH};
use crate::model::player::Player;
use crate::model::territory::Territory;
use crate::render::army_placement::army_placement;
use crate::render::city_selection::city_selection;
use crate::render::lower_panel::render_lower_panel;
use crate::render::randomising::randomising;
use crate::render::region_summary::region_summary;
use crate::render::surround::render_surround;
use crate::render::title_bar::render_title_bar;
use skia_safe::textlayout::TextAlign;
use skia_safe::{dash_path_effect, Color, Paint, PaintStyle, Point, RRect, Rect};
use specs::{Join, WorldExt};

fn render_connections(skia: &mut Skia, app_state: &mut AppState) {
    // Paint
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::YELLOW);
    paint.set_stroke_width(LINE_WIDTH);
    paint.set_style(PaintStyle::Stroke);
    let mut paint_alt = Paint::default();
    paint_alt.set_anti_alias(true);
    paint_alt.set_color(Color::BLACK);
    paint_alt.set_stroke_width(LINE_WIDTH);
    paint_alt.set_style(PaintStyle::Stroke);
    let phase = 0.0;
    paint.set_path_effect(dash_path_effect::new(&[1.0, 1.0], phase).unwrap());
    paint_alt.set_path_effect(dash_path_effect::new(&[1.0, 1.0], phase + 1.0).unwrap());

    let cities = app_state.world.read_storage::<City>();
    let connections = app_state.world.read_storage::<Connection>();
    for connection in connections.join() {
        let city1 = cities.get(connection.city1).unwrap();
        let city2 = cities.get(connection.city2).unwrap();

        skia.get_canvas().draw_line(city1.location.p, city2.location.p, &paint);
        skia.get_canvas().draw_line(city1.location.p, city2.location.p, &paint_alt);
    }
}

fn render_territories(skia: &mut Skia, app_state: &mut AppState) {
    let territories = app_state.world.read_storage::<Territory>();
    for territory in territories.join() {
        for polygon in &territory.polygons {
            skia.get_canvas().draw_picture(polygon.pic.as_ref(), None, None);
        }
    }
}

fn render_cities(skia: &mut Skia, app_state: &mut AppState) {
    let cities = app_state.world.read_storage::<City>();
    let territories = app_state.world.read_storage::<Territory>();
    for territory in territories.join() {
        for city_state in &territory.cities {
            let selected = if let Some(selected) = &app_state.selection.last_city_selection {
                if app_state.current_player == app_state.actual_human {
                    selected == city_state
                } else { false }
            } else {
                false
            };
            let hover = if let Some(hover) = &app_state.selection.last_city_hover {
                if app_state.current_player == app_state.actual_human {
                    hover == city_state
                } else { false }
            } else {
                false
            };
            let city = cities.get(city_state.city).unwrap();
            let centre = city.location.p;
            let territory = territories.get(city.territory).unwrap();
            let font_size: f32 = 2.4;

            let mut paint_name = Paint::default();
            paint_name.set_anti_alias(true);
            paint_name.set_style(PaintStyle::Fill);
            paint_name.set_color(Color::BLACK);
            let mut paint_shadow = Paint::default();
            paint_shadow.set_style(PaintStyle::Fill);
            paint_shadow.set_image_filter(skia.drop_shadow.clone());
            let mut paint_fill = Paint::default();
            paint_fill.set_style(PaintStyle::Fill);
            paint_fill.set_color(skia::mix_colors(territory.colour, Color::WHITE, 0.6));
            let mut paint_fill_circle = Paint::default();
            paint_fill_circle.set_style(PaintStyle::Fill);
            let colours = match &city_state.owner {
                Some(x) => app_state.world.read_storage::<Player>().get(*x).unwrap().colours.clone(),
                None => vec![Color::from_rgb(128, 128, 128), Color::BLACK],
            };
            paint_fill_circle.set_color(colours[0]);
            let mut paint_number = Paint::default();
            paint_number.set_anti_alias(true);
            paint_number.set_style(PaintStyle::Fill);
            paint_number.set_color(colours[1]);
            let mut paint_outline = Paint::default();
            paint_outline.set_anti_alias(true);
            paint_outline.set_style(PaintStyle::Stroke);
            paint_outline.set_color(Color::BLACK);
            paint_outline.set_stroke_width(SIZE / 8.0);

            // Name background
            if app_state.show_all_info() {
                let dimensions = skia
                    .text_dimensions(font_size, &paint_name, &city.name, &FontFamily::EbGaramond, TextAlign::Left)
                    .clamp(1.0, MAXIMUM_LABEL_WIDTH);
                if app_state.show_shadows {
                    skia.get_canvas().draw_round_rect(
                        Rect::from_xywh(centre.x, centre.y - 1.5, dimensions + SIZE + 1.5, 3.0),
                        0.5,
                        0.5,
                        &paint_shadow,
                    );
                }
                skia.get_canvas().draw_round_rect(
                    Rect::from_xywh(centre.x, centre.y - 1.5, dimensions + SIZE + 1.5, 3.0),
                    0.5,
                    0.5,
                    &paint_fill,
                );
                skia.get_canvas().draw_round_rect(
                    Rect::from_xywh(centre.x, centre.y - 1.5, dimensions + SIZE + 1.5, 3.0),
                    0.5,
                    0.5,
                    &paint_outline,
                );
            }

            // Draw
            if app_state.show_shadows {
                skia.get_canvas().draw_circle(centre, SIZE, &paint_shadow);
            }
            skia.get_canvas().draw_circle(centre, SIZE, &paint_fill_circle);
            if hover {
                paint_outline.set_color(colours[1]);
                paint_outline.set_path_effect(dash_path_effect::new(&[0.5, 0.5], app_state.phase).unwrap());
            }
            skia.get_canvas().draw_circle(centre, SIZE, &paint_outline);
            let strength = format!("{}/{}", city_state.armies, city.size);
            skia.write_text_centre(
                5.0,
                &paint_number,
                &strength,
                Point::new(centre.x - SIZE, centre.y - 3.5),
                SIZE * 2.0,
                &FontFamily::EbGaramond,
            );
            if app_state.show_all_info() {
                skia.write_text(
                    font_size,
                    &paint_name,
                    &city.name,
                    Point::new(centre.x + SIZE + 0.5, centre.y - 1.2),
                    MAXIMUM_LABEL_WIDTH,
                    &FontFamily::EbGaramond,
                );
            }
            if selected {
                let mut paint_selected = Paint::default();
                paint_selected.set_anti_alias(true);
                paint_selected.set_style(PaintStyle::Stroke);
                paint_selected.set_color(Color::WHITE);
                paint_selected.set_stroke_width(SIZE / 4.0);
                paint_selected.set_path_effect(dash_path_effect::new(&[5.0, 5.0], app_state.phase * 5.0).unwrap());
                skia.get_canvas().draw_circle(centre, SIZE_SELECTED, &paint_selected);
            }
        }
    }
}

pub fn main(skia: &mut Skia, app_state: &mut AppState) {
    skia.reset_context();

    let clip_rect = RRect::new_rect_xy(
        Rect::from_xywh(32.0, 32.0, app_state.gfx.width as f32 - 64.0, app_state.gfx.height as f32 - 64.0),
        128.0,
        128.0,
    );
    let canvas = skia.get_canvas();
    canvas.save();
    canvas.reset_matrix();
    canvas.scale((app_state.gfx.dpi, app_state.gfx.dpi));

    // Clip the edges
    canvas.clip_rrect(clip_rect, None, true);

    // Transforms
    canvas.translate((app_state.gfx.half_width, app_state.gfx.half_height));
    canvas.scale((app_state.zoom, app_state.zoom));
    canvas.translate((-app_state.target.x, -app_state.target.y));

    // Now render parts
    render_territories(skia, app_state);
    render_connections(skia, app_state);
    render_cities(skia, app_state);

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
        "FPS: {:.0} Zoom: {}, Position: {},{}",
        app_state.fps, app_state.zoom, app_state.target.x, app_state.target.y
    );
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(skia_safe::Color::WHITE);
    /*skia.write_text(20.0, &paint, fps.as_str(), Point::new(0.0, 0.0), 0.0,
    &FontFamily::EbGaramond);*/

    // Flush all Skia ops
    unsafe {
        skia.flush();
    }
}
