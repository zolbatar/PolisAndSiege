use crate::model::math::{degrees_to_radians, radians_to_degrees};
use skia_safe::Point;
use specs::prelude::*;
use specs_derive::Component;
use std::f32::consts::PI;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct CLocation {
    pub latitude: f32,
    pub longitude: f32,
    pub p: Point,
}

pub struct SConvertXYToLatLong;
impl<'a> System<'a> for SConvertXYToLatLong {
    type SystemData = WriteStorage<'a, CLocation>;

    fn run(&mut self, mut components: Self::SystemData) {
        for component in (&mut components).join() {
            component.latitude = component.latitude.clamp(-89.5, 89.5);
            let y: f32 = radians_to_degrees(f32::ln(f32::tan(PI / 4.0 + degrees_to_radians(component.latitude) / 2.0)));
            component.p = Point::new(component.longitude, y);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
    pub p: Point,
}

impl Location {
    pub fn new(longitude: f32, mut latitude: f32) -> Self {
        latitude = latitude.clamp(-89.5, 89.5);
        let y: f32 = radians_to_degrees(f32::ln(f32::tan(PI / 4.0 + degrees_to_radians(latitude) / 2.0)));
        Location {
            latitude: latitude,
            longitude: longitude,
            p: Point::new(longitude, y),
        }
    }
}

pub fn calculate_distance(city1: &Location, city2: &Location) -> f32 {
    let dx = city1.p.x - city2.p.x;
    let dy = city1.p.y - city2.p.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn calculate_distance_new(city1: &CLocation, city2: &CLocation) -> f32 {
    let dx = city1.p.x - city2.p.x;
    let dy = city1.p.y - city2.p.y;
    (dx * dx + dy * dy).sqrt()
}
