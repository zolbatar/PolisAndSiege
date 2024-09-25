use crate::model::location::Location;
use crate::skia;
use crate::skia::Skia;
use lazy_static::lazy_static;
use skia_safe::{Canvas, Color, Paint, PaintStyle, Point};
use std::sync::Mutex;

pub enum CityType {
    Metropolis,
    Fortopolis,
    Argopolis,
    Technopolis,
}

pub struct City {
    name: String,
    location: Location,
    population: i64,
    paint_territory: Color,
    size: i8,
    typ: CityType,
}

const SIZE: f32 = 2.0;
const MINIMUM_ALLOWED_DISTANCE: f32 = 12.0;
lazy_static! {
    static ref EXISTING_CITIES: Mutex<Vec<Location>> = Mutex::new(Vec::new());
}

impl City {
    pub fn new(name: String, latitude: f32, longitude: f32, population: i64, paint_territory: Color) -> Self {
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
            location: Location::new(latitude, longitude),
            population,
            paint_territory,
            size,
            typ: CityType::Metropolis,
        }
    }

    pub fn render(&self, canvas: &Canvas, skia: &Skia) {
        let mut paint = Paint::default();
        paint.set_anti_alias(false);
        paint.set_style(PaintStyle::Fill);

        let centre = Point::new(self.location.x, self.location.y);

        // Draw
        let mut paint_fill = Paint::default();
        paint_fill.set_style(PaintStyle::Fill);
        paint_fill.set_color(skia::mix_colors(self.paint_territory, Color::WHITE, 0.5));
        canvas.draw_circle(centre, SIZE, &paint_fill);

        // Outline
        let mut paint_outline = Paint::default();
        paint_outline.set_anti_alias(true);
        paint_outline.set_style(PaintStyle::Stroke);
        paint_outline.set_color(Color::BLACK);
        paint_outline.set_stroke_width(SIZE / 8.0);
        canvas.draw_circle(centre, SIZE, &paint_outline);

        // Size label
        let mut paint_text = Paint::default();
        paint_text.set_anti_alias(true);
        paint_text.set_style(PaintStyle::Fill);
        paint_text.set_color(Color::BLACK);
        skia.write_text_centre(canvas, 3.0, &paint_text, &self.size.to_string(), Point::new(centre.x, centre.y - SIZE), self.location.y);
    }

    pub fn calculate_distance(city1: &City, city2: &City) -> f32 {
        Location::calculate_distance(&city1.location, &city2.location)
    }

    // Function to select evenly spaced cities
    pub fn select_evenly_spaced_cities(
        mut cities: Vec<City>,
        num_cities_to_select: usize,
    ) -> Vec<City> {
        let mut selected_cities: Vec<City> = Vec::new();

        // Sort the cities by population (largest first)
        cities.sort_by(|a, b| b.population.cmp(&a.population)); // Sort largest first

        // Loop through all cities
        for city in cities {
            let mut want = true;

            // Check distance to already selected cities
            let mut existing_cities = EXISTING_CITIES.lock().unwrap();
            for existing in existing_cities.iter() {
                if existing.x != city.location.x && existing.y != city.location.y {
                    let dist = Location::calculate_distance(&city.location, existing);
                    if dist <= MINIMUM_ALLOWED_DISTANCE {
                        want = false;
                        break;
                    }
                }
            }

            // If the city is far enough, select it
            if want {
                existing_cities.push(city.location.clone());
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
