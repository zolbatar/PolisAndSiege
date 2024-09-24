use skia_safe::Canvas;
use crate::model::city::City;
use crate::model::territory_polygon::TerritoryPolygon;

pub struct Territory {
    name: String,
    pub polygons: Vec<TerritoryPolygon>,
    pub cities: Vec<City>,
}

impl Territory {
    pub fn new(name: &String) -> Self {
        let name_cloned = name.clone();
        Territory {
            name: name_cloned,
            polygons: Vec::new(),
            cities: Vec::new(),
        }
    }

    pub fn prerender_polygons(&mut self) {
        for polygon in &mut self.polygons {
            polygon.prerender();
        }
    }

    pub fn render_polygons(&self, canvas: &Canvas) {
        for polygon in &self.polygons {
            polygon.render(canvas);
        }
    }
}
