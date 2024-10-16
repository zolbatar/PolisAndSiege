use crate::lib::skia::Skia;
use crate::model::location::Location;
use skia_safe::{Color, Paint, PaintStyle, Path, Picture, PictureRecorder, Rect};

#[derive(Debug)]
pub struct TerritoryPolygon {
    pub pic: Picture,
}

impl TerritoryPolygon {
    pub fn new(_skia: &mut Skia, colour: Color, locations: Vec<Location>) -> Self {
        // Paint
        let mut paint = Paint::default();
        paint.set_style(PaintStyle::Fill);
        paint.set_argb(255, 255, 0, 0);
        paint.set_color(colour);

        // Construct path
        let mut path = Path::new();
        path.move_to(locations[0].p);
        for location in locations.iter().skip(1) {
            path.line_to(location.p);
        }
        path.close();

        // And draw to a Picture
        let mut recorder = PictureRecorder::new();
        let canvas = recorder.begin_recording(Rect::from_wh(0.0, 0.0), None);
        //paint.set_shader(skia.halftone(territories.get(territory).unwrap().colour));
        canvas.draw_path(&path, &paint);
        let pic = recorder.finish_recording_as_picture(None).unwrap();

        Self {
            pic,
        }
    }
}
