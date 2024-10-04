use crate::model::city::City;
use crate::model::territory_polygon::TerritoryPolygon;
use skia_safe::{Canvas, Color};
use std::sync::{Arc, Mutex};

pub struct Territory {
    pub name: String,
    pub colour: Color,
    pub polygons: Vec<TerritoryPolygon>,
    pub cities: Vec<Arc<Mutex<City>>>,
}

impl Territory {
    pub fn new(name: &String) -> Self {
        let name_cloned = name.clone();
        Territory {
            name: name_cloned,
            polygons: Vec::new(),
            cities: Vec::new(),
            colour: Color::TRANSPARENT,
        }
    }

    pub fn prerender_polygons(&mut self) -> Color {
        let colour = match self.name.as_str() {
            "Europe" => Color::from_argb(255, 46, 92, 229),
            "Eastern Europe" => Color::from_argb(255, 255, 78, 69),
            "Asia" => Color::from_argb(255, 50, 205, 50),
            "Sub-Saharan Africa" => Color::from_argb(255, 255, 204, 0),
            "Middle East and North Africa" => Color::from_argb(255, 0, 139, 139),
            "Australia and New Zealand" => Color::from_argb(255, 181, 101, 29),
            "Latin America and the Caribbean" => Color::from_argb(255, 255, 140, 0),
            "North America" => Color::from_argb(255, 219, 112, 147),
            &_ => todo!(),
        };
        self.colour = colour;

        for polygon in &mut self.polygons {
            polygon.prerender(colour);
        }
        colour
    }

    pub fn render_polygons(&self, canvas: &Canvas) {
        for polygon in &self.polygons {
            polygon.render(canvas);
        }
    }
}
