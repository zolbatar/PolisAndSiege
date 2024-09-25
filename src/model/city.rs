use crate::model::location::Location;
use skia_safe::colors::BLACK;
use skia_safe::{Canvas, Color, Paint, PaintStyle, Point};
use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct City {
    name: String,
    location: Location,
    population: i64,
    paint_territory: Color,
}

const MIN_SIZE: f32 = 1.0;
const MAX_SIZE: f32 = 2.5;
const MINIMUM_ALLOWED_DISTANCE: f32 = 12.0;
lazy_static! {
    static ref EXISTING_CITIES: Mutex<Vec<Location>> = Mutex::new(Vec::new());
}

impl City {
    pub fn new(name: String, latitude: f32, longitude: f32, population: i64, paint_territory: Color) -> Self {
        City {
            name,
            location: Location::new(latitude, longitude),
            population,
            paint_territory,
        }
    }

    pub fn render(&self, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_anti_alias(false);
        paint.set_style(PaintStyle::Fill);

        // Size
        let size = City::log_transform(self.population as f32).clamp(MIN_SIZE, MAX_SIZE);

        // Draw
        let mut paint_fill = Paint::default();
        paint_fill.set_style(PaintStyle::Fill);
        paint_fill.set_color(self.paint_territory);
        //paint_fill.setColor(Skia::MixColors(paint_territory, SkColors::kWhite.toSkColor(), 0.5f));
        paint_fill.set_image_filter(None);
        canvas.draw_circle(Point::new(self.location.x, self.location.y), size, &paint_fill);

        // Outline
        let mut paint_outline = Paint::default();
        paint_outline.set_anti_alias(true);
        paint_outline.set_style(PaintStyle::Stroke);
        paint_outline.set_color4f(BLACK, None);
        paint_outline.set_stroke_width(size / 8.0);
        canvas.draw_circle(Point::new(self.location.x, self.location.y), size, &paint_outline);
    }

    fn log_transform(x: f32) -> f32 {
        const MIN_INPUT: f32 = 100_000.0;
        const MAX_INPUT: f32 = 25_000_000.0;

        // Calculate log_min and log_max for normalization
        let log_min = MIN_INPUT.ln(); // ln() is the natural logarithm in Rust
        let log_max = MAX_INPUT.ln();

        // Calculate the log of the input value (no need to add 1 if x is non-zero)
        let log_x = x.ln();

        // Normalize log_x to a range between 0 and 1
        let normalized_log_x = (log_x - log_min) / (log_max - log_min);

        // Scale and shift to the desired output range
        let output = MIN_SIZE + normalized_log_x * (MAX_SIZE - MIN_SIZE);

        output
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
