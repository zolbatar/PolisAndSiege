use crate::app_state::AppState;
use crate::lib::skia;
use crate::lib::skia::{FontFamily, Skia};
use crate::model::location::Location;
use crate::model::territory::Territory;
use petgraph::graph::NodeIndex;
use skia_safe::textlayout::TextAlign;
use skia_safe::{Color, Paint, PaintStyle, Point, Rect};
use std::sync::{Arc, Mutex};

pub enum CityType {
    City,
    Metropolis,
    Fortopolis,
}

#[derive(Eq, Hash, PartialEq, Clone, Ord, PartialOrd)]
pub enum Owner {
    None,
    Player,
    Enemy1,
    Enemy2,
    Enemy3,
    Enemy4,
}

pub struct City {
    pub territory: Arc<Mutex<Territory>>,
    pub name: String,
    pub location: Location,
    population: i64,
    paint_territory: Color,
    typ: CityType,
    size: u8,
    fractional_size: f32,
    armies: u8,
    pub node: NodeIndex,
    pub owner: Owner,
}

pub const SIZE: f32 = 3.0;
const MAXIMUM_LABEL_WIDTH: f32 = 32.0;

impl City {
    pub fn new(name: String, longitude: f32, latitude: f32, population: i64, territory: Arc<Mutex<Territory>>) -> Self {
        let size = match population {
            0..150000 => 1,
            150000..250000 => 2,
            250000..500000 => 3,
            500000..1000000 => 4,
            1000000..2500000 => 5,
            2500000..5000000 => 6,
            5000000..10000000 => 7,
            _ => 8, /*
                                0..150000 => 1,
                    250000..500000 => 2,
                    1000000..2500000 => 3,
                    _ => 4

                     */
        };
        City {
            territory: territory.clone(),
            name,
            location: Location::new(longitude, latitude),
            population,
            paint_territory: territory.lock().unwrap().colour,
            typ: CityType::City,
            size,
            fractional_size: size as f32,
            armies: 1,
            node: NodeIndex::new(0),
            owner: Owner::None,
        }
    }

    pub fn render(&self, skia: &mut Skia, app_state: &AppState) {
        let centre = self.location.p;
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
        paint_fill.set_color(skia::mix_colors(self.paint_territory, Color::WHITE, 0.6));
        let mut paint_fill_circle = Paint::default();
        paint_fill_circle.set_style(PaintStyle::Fill);
        let colours = app_state.res.player_colours.get(&self.owner).unwrap();
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
                .text_dimensions(font_size, &paint_name, &self.name, &FontFamily::EbGaramond, TextAlign::Left)
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
        skia.get_canvas().draw_circle(centre, SIZE, &paint_outline);
        let strength = format!("{}/{}", self.armies, self.size);
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
                &self.name,
                Point::new(centre.x + SIZE + 0.5, centre.y - 1.2),
                MAXIMUM_LABEL_WIDTH,
                &FontFamily::EbGaramond,
            );
        }
    }

    pub fn calculate_distance(city1: &City, city2: &City) -> f32 {
        Location::calculate_distance(&city1.location, &city2.location)
    }

    // Function to select evenly spaced cities
    pub fn select_evenly_spaced_cities(
        app_state: &mut AppState,
        territory: Arc<Mutex<Territory>>,
        num_cities_to_select: usize,
    ) {
        let mut selected_cities: Vec<Arc<Mutex<City>>> = Vec::new();

        // Sort the cities by population (largest first)
        let mut cities = territory.lock().unwrap().cities.clone();
        cities.sort_by(|a, b| b.lock().unwrap().population.cmp(&a.lock().unwrap().population)); // Sort largest first

        // Loop through all cities
        for city in cities {
            let mut want = true;

            // Check distance to already selected cities
            for existing in app_state.items.existing_cities.iter() {
                if existing.p != city.lock().unwrap().location.p {
                    let dist = Location::calculate_distance(&city.lock().unwrap().location, existing);
                    if dist <= app_state.selection.minimum_allowed_distance {
                        want = false;
                        break;
                    }
                }
            }

            // If the city is far enough, select it
            if want {
                app_state.items.existing_cities.push(city.lock().unwrap().location.clone());
                selected_cities.push(city.clone());

                // Stop if we have selected enough cities
                if selected_cities.len() >= num_cities_to_select {
                    territory.lock().unwrap().cities = selected_cities;
                    return;
                }
            }
        }

        territory.lock().unwrap().cities = selected_cities;
    }
}
