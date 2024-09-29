use crate::app_state::AppState;
use crate::lib::skia;
use crate::lib::skia::Skia;
use crate::model::location::Location;
use petgraph::graph::NodeIndex;
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};
use std::sync::{Arc, Mutex};

pub enum CityType {
    Metropolis,
    Fortopolis,
    Argopolis,
    Technopolis,
}

pub struct City {
    pub name: String,
    pub location: Location,
    population: i64,
    paint_territory: Color,
    size: i8,
    typ: CityType,
    pub node: NodeIndex,
}

const SIZE: f32 = 2.0;
const MINIMUM_ALLOWED_DISTANCE: f32 = 12.0;
const MAXIMUM_LABEL_WIDTH: f32 = 12.0;

impl City {
    pub fn new(name: String, longitude: f32, latitude: f32, population: i64, paint_territory: Color) -> Self {
        let size = match population {
            0..150000 => 1,
            150000..250000 => 2,
            250000..500000 => 3,
            500000..1000000 => 4,
            1000000..2500000 => 5,
            2500000..5000000 => 6,
            5000000..10000000 => 7,
            _ => 8
        };
        City {
            name,
            location: Location::new(longitude, latitude),
            population,
            paint_territory,
            size,
            typ: CityType::Metropolis,
            node: NodeIndex::new(0),
        }
    }

    pub fn render(&self, skia: &mut Skia, app_state: &AppState) {
        let mut paint = Paint::default();
        paint.set_anti_alias(false);
        paint.set_style(PaintStyle::Fill);

        let centre = self.location.p;
        let font_size: f32 = 2.0;

        let mut paint_name = Paint::default();
        paint_name.set_anti_alias(true);
        paint_name.set_style(PaintStyle::Fill);
        paint_name.set_color(Color::BLACK);
        let mut paint_shadow = Paint::default();
        paint_shadow.set_style(PaintStyle::Fill);
        paint_shadow.set_image_filter(skia.drop_shadow.clone());
        let mut paint_fill = Paint::default();
        paint_fill.set_style(PaintStyle::Fill);
        paint_fill.set_color(skia::mix_colors(self.paint_territory, Color::WHITE, 0.7));
        let mut paint_outline = Paint::default();
        paint_outline.set_anti_alias(true);
        paint_outline.set_style(PaintStyle::Stroke);
        paint_outline.set_color(Color::BLACK);
        paint_outline.set_stroke_width(SIZE / 8.0);

        // Name background
        if app_state.show_all_info() {
            let dimensions = skia.text_dimensions(font_size, &paint_name, &self.name).clamp(1.0, MAXIMUM_LABEL_WIDTH);
            skia.get_canvas().draw_round_rect(Rect::from_xywh(centre.x, centre.y - SIZE / 2.0 - 0.5, dimensions + SIZE + 1.5, 3.0), 0.5, 0.5, &paint_fill);
            skia.get_canvas().draw_round_rect(Rect::from_xywh(centre.x, centre.y - SIZE / 2.0 - 0.5, dimensions + SIZE + 1.5, 3.0), 0.5, 0.5, &paint_outline);
        }

        // Draw
//        canvas.draw_circle(centre, SIZE, &paint_shadow);
        skia.get_canvas().draw_circle(centre, SIZE, &paint_fill);
        skia.get_canvas().draw_circle(centre, SIZE, &paint_outline);
        skia.write_text_centre(3.0, &paint_name, &self.size.to_string(), Point::new(centre.x, centre.y - SIZE - 0.1), 0.0);
        if app_state.show_all_info() {
            skia.write_text(font_size, &paint_name, &self.name, Point::new(centre.x + SIZE + 0.5, centre.y - font_size / 1.5), MAXIMUM_LABEL_WIDTH);
        }
    }

    pub fn calculate_distance(city1: &City, city2: &City) -> f32 {
        Location::calculate_distance(&city1.location, &city2.location)
    }

    // Function to select evenly spaced cities
    pub fn select_evenly_spaced_cities(
        app_state: &mut AppState,
        mut cities: Vec<Arc<Mutex<City>>>,
        num_cities_to_select: usize,
    ) -> Vec<Arc<Mutex<City>>> {
        let mut selected_cities: Vec<Arc<Mutex<City>>> = Vec::new();

        // Sort the cities by population (largest first)
        cities.sort_by(|a, b| b.lock().unwrap().population.cmp(&a.lock().unwrap().population)); // Sort largest first

        // Loop through all cities
        for city in cities {
            let mut want = true;

            // Check distance to already selected cities
            for existing in app_state.existing_cities.iter() {
                if existing.p != city.lock().unwrap().location.p {
                    let dist = Location::calculate_distance(&city.lock().unwrap().location, existing);
                    if dist <= MINIMUM_ALLOWED_DISTANCE {
                        want = false;
                        break;
                    }
                }
            }

            // If the city is far enough, select it
            if want {
                app_state.existing_cities.push(city.lock().unwrap().location.clone());
                selected_cities.push(city);

                // Stop if we have selected enough cities
                if selected_cities.len() >= num_cities_to_select {
                    return selected_cities;
                }
            }
        }

        selected_cities
    }
}
