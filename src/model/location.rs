use std::f32::consts::PI;

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

fn radians_to_degrees(radians: f32) -> f32 {
    radians * (180.0 / PI)
}

pub struct Location {
    latitude: f32,
    longitude: f32,
    pub x: f32,
    pub y: f32,
}

impl Location {
    pub fn new(mut latitude: f32, longitude: f32) -> Self {
        latitude = latitude.max(-89.5).min(89.5);
        let y: f32 = radians_to_degrees(f32::ln(f32::tan(PI / 4.0 + degrees_to_radians(latitude) / 2.0)));
        Location {
            latitude,
            longitude,
            x: longitude,
            y,
        }
    }
}