use std::f32::consts::PI;
use crate::model::math::{degrees_to_radians, radians_to_degrees};

#[derive(Clone)]
pub struct Location {
    _latitude: f32,
    _longitude: f32,
    pub x: f32,
    pub y: f32,
}

impl Location {
    pub fn new(mut latitude: f32, longitude: f32) -> Self {
        latitude = latitude.clamp(-89.5, 89.5);
        let y: f32 = radians_to_degrees(f32::ln(f32::tan(PI / 4.0 + degrees_to_radians(latitude) / 2.0)));
        Location {
            _latitude: latitude,
            _longitude: longitude,
            x: longitude,
            y,
        }
    }

    pub fn calculate_distance(city1: &Location, city2: &Location) -> f32 {
        let dx = city1.x - city2.x;
        let dy = city1.y - city2.y;
        (dx * dx + dy * dy).sqrt()
    }
}
