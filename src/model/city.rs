use crate::model::location::Location;

pub struct City {
    name: String,
    location: Location,
    population: i64,
}

impl City {
    pub fn new(name: String, latitude: f32, longitude: f32, population: i64) -> Self {
        City {
            name,
            location: Location::new(latitude, longitude),
            population,
        }
    }
}