use crate::model::city::City;
use crate::model::territory_polygon::TerritoryPolygon;

pub struct Territory {
    name: String,
    pub polygons: Vec<TerritoryPolygon>,
    pub cities: Vec<City>,
}

impl Territory {
    pub fn new(name: String) -> Self {
        Territory {
            name,
            polygons: Vec::new(),
            cities: Vec::new(),
        }
    }
}