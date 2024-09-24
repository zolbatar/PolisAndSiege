use skia_safe::{Canvas, Paint, PaintStyle, Path, Point};
use crate::model::location::Location;

pub struct TerritoryPolygon {
    pub locations: Vec<Location>,
    path: Option<Path>,
    paint: Paint,
}

impl TerritoryPolygon {
    pub fn new() -> Self {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_argb(255, 255, 0, 0);
        TerritoryPolygon {
            locations: Vec::new(),
            path: None,
            paint,
        }
    }

    pub fn prerender(&mut self) {
        let mut path = Path::new();
        path.move_to(Point::new(self.locations[0].x, self.locations[0].y));
        for location in self.locations.iter().skip(1) {
            path.line_to(Point::new(location.x, location.y));
        }
        path.close();

        self.path = Some(path);
    }

    pub fn render(&self, canvas: &Canvas) {
        canvas.draw_path(self.path.as_ref().unwrap(), &self.paint);
    }
}