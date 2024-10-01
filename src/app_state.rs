use std::collections::HashMap;
use sdl2::video::Window;
use skia_safe::{FontMgr, Path, Point, Size};
use crate::model::connection::Connection;
use crate::model::location::Location;
use crate::model::territory::Territory;
use skia_safe::svg::Dom;

const SVG_CORNER: &str = include_str!("../assets/Corner.svg");
const SVG_SIDE: &str = include_str!("../assets/Side.svg");

pub(crate) const MIN_ZOOM: f32 = 5.0;

pub struct AppState {
    pub fps: f64,
    pub width: i32,
    pub height: i32,
    pub half_width: i32,
    pub half_height: i32,
    pub dpi: f32,
    pub zoom: f32,
    pub target: Point,
    pub panning: bool,
    pub territories: HashMap<String, Territory>,
    pub existing_cities: Vec<Location>,
    pub connections: Vec<Connection>,
    pub show_labels: bool,
    pub side_path: Dom,
    pub corner_path: Dom,
}

impl AppState {
    pub fn new(window: &Window, dpi: f32) -> Self {
        let width = window.size().0 as i32;
        let height = window.size().1 as i32;
        println!("Screen resolution: {}x{}", width, height);
        let half_width = width / 2;
        let half_height = height / 2;

        let st = "";
        Path::from_svg(st);
        let mut corner_path = Dom::from_str(SVG_CORNER, FontMgr::new()).expect("Error loading SVG");
        corner_path.set_container_size(Size::new(160.0, 160.0));
        let mut side_path = Dom::from_str(SVG_SIDE, FontMgr::new()).expect("Error loading SVG");
        side_path.set_container_size(Size::new(40.0, 200.0));

        AppState {
            fps: 0.0,
            width,
            height,
            half_width,
            half_height,
            dpi,
            territories: HashMap::new(),
            panning: false,
            existing_cities: Vec::new(),
            connections: Vec::new(),
            show_labels: true,
            zoom: MIN_ZOOM,
            target: Point::new(25.0, -10.0),
            corner_path,
            side_path,
        }
    }

    pub fn show_all_info(&self) -> bool {
        self.show_labels
    }
}