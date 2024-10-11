use crate::model::location::CLocation;
use crate::model::territory::CTerritory;
use skia_safe::{Paint, PaintStyle, Path, Picture, PictureRecorder, Rect};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CTerritoryPolygon {
    pub territory: Entity,
    pub locations: Vec<CLocation>,
    pub pic: Option<Picture>,
}

pub struct SCreatePictureForPolygon;

impl<'a> System<'a> for SCreatePictureForPolygon {
    type SystemData = (WriteStorage<'a, CTerritoryPolygon>, ReadStorage<'a, CTerritory>, ReadStorage<'a, CLocation>);

    fn run(&mut self, (mut components, territories, locations): Self::SystemData) {
        for component in (&mut components).join() {
            let territory = territories.get(component.territory).unwrap();

            // Paint
            let mut paint = Paint::default();
            //        paint.set_anti_alias(true);
            paint.set_style(PaintStyle::Fill);
            paint.set_argb(255, 255, 0, 0);
            paint.set_color(territory.colour);

            // Construct path
            let mut path = Path::new();
            path.move_to(component.locations[0].p);
            for location in component.locations.iter().skip(1) {
                path.line_to(location.p);
//                println!("{:?}", location.p);
            }
            path.close();

            // And draw to a Picture
            let mut recorder = PictureRecorder::new();
            let canvas = recorder.begin_recording(Rect::from_wh(0.0, 0.0), None);
            canvas.draw_path(&path, &paint);
            component.pic = recorder.finish_recording_as_picture(None);
        }
    }
}
