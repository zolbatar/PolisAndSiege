use crate::model::location::Location;
use crate::model::territory::Territory;
use skia_safe::{Paint, PaintStyle, Path, Picture, PictureRecorder, Rect};
use specs::prelude::*;
use specs_derive::Component;
use crate::app_state::AppState;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TerritoryPolygon {
    pub pic: Picture,
}

impl TerritoryPolygon {
    pub fn new(app_state: &mut AppState, territory: Entity, locations: Vec<Location>) -> Self {
        let territories = app_state.world.read_storage::<Territory>();

        // Paint
        let mut paint = Paint::default();
        //        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_argb(255, 255, 0, 0);
        paint.set_color(territories.get(territory).unwrap().colour);

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
        canvas.draw_path(&path, &paint);
        let pic = recorder.finish_recording_as_picture(None).unwrap();

        Self {
            pic,
        }
    }
}
