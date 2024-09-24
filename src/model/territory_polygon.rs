use crate::model::location::Location;

pub struct TerritoryPolygon {
    pub locations: Vec<Location>,
}

impl TerritoryPolygon {
    pub fn new() -> Self {
        TerritoryPolygon {
            locations: Vec::new()
        }
    }
}