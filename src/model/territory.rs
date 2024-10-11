use skia_safe::Color;
use specs::prelude::*;
use specs_derive::Component;
use crate::model::territory_polygon::TerritoryPolygon;

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

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Territory {
    pub cities: Vec<Entity>,
    pub polygons: Vec<TerritoryPolygon>,
    pub name: String,
    pub colour: Color,
}
