use crate::model::city::CityRR;
use crate::model::territory_polygon::TerritoryPolygon;
use skia_safe::Color;
use std::sync::{Arc};

pub fn get_colour_for_territory_name(name: &String) -> Color {
    match name.as_str() {
        "Europe" => Color::from_argb(255, 46, 92, 229),
        "Eastern Europe" => Color::from_argb(255, 255, 78, 69),
        "Asia" => Color::from_argb(255, 50, 205, 50),
        "Sub-Saharan Africa" => Color::from_argb(255, 255, 204, 0),
        "Middle East and North Africa" => Color::from_argb(255, 0, 139, 139),
        "Australia and New Zealand" => Color::from_argb(255, 181, 101, 29),
        "Latin America and the Caribbean" => Color::from_argb(255, 255, 140, 0),
        "North America" => Color::from_argb(255, 219, 112, 147),
        &_ => todo!(),
    }
}

#[derive(Debug, Default)]
pub struct Territory {
    pub cities: Vec<CityRR>,
    pub polygons: Vec<TerritoryPolygon>,
    pub name: String,
    pub colour: Color,
}

pub type TerritoryArc = Arc<Territory>;

impl Territory {
    pub fn containerise(self) -> TerritoryArc {
        let container = Arc::new(self);

        // Update cities
        container.cities.iter().for_each(|city| city.borrow_mut().territory = container.clone());

        container
    }
}
