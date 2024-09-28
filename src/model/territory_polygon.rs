use skia_safe::{Canvas, Color, Paint, PaintStyle, Path, Picture, PictureRecorder, Rect};
use crate::model::location::Location;

pub struct TerritoryPolygon {
    pub locations: Vec<Location>,
    path: Option<Path>,
    paint: Paint,
    pic: Option<Picture>,
}

impl TerritoryPolygon {
    pub fn new() -> Self {
        let mut paint = Paint::default();
        //        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_argb(255, 255, 0, 0);
        TerritoryPolygon {
            locations: Vec::new(),
            path: None,
            paint,
            pic: None,
        }
    }

    pub fn prerender(&mut self, color: Color) {
        self.paint.set_color(color);
        let mut path = Path::new();
        path.move_to(self.locations[0].p);
        for location in self.locations.iter().skip(1) {
            path.line_to(location.p);
        }
        path.close();

        self.path = Some(path);

        let mut recorder = PictureRecorder::new();
        let canvas = recorder.begin_recording(Rect::from_wh(0.0, 0.0), None);
        canvas.draw_path(self.path.as_ref().unwrap(), &self.paint);
        self.pic = recorder.finish_recording_as_picture(None);
    }

    pub fn render(&self, canvas: &Canvas) {
        canvas.draw_picture(self.pic.as_ref().unwrap(), None, None);
    }
}