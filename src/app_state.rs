use std::collections::HashMap;
use sdl2::video::Window;
use skia_safe::{Path, Point};
use crate::model::connection::Connection;
use crate::model::location::Location;
use crate::model::territory::Territory;

const SVG_CORNER: &str = include_str!("../assets/Corner.svg");
const SVG_SIDE: &str = include_str!("../assets/Side.svg");

pub(crate) const MIN_ZOOM: f32 = 3.8375;

pub struct AppState {
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
    pub side_path: Path,
    pub corner_path: Path,
}

impl AppState {
    pub fn new(window: &Window, dpi: f32) -> Self {
        let width = window.size().0 as i32;
        let height = window.size().1 as i32;
        let half_width = width / 2;
        let half_height = height / 2;

        let st = "";
        Path::from_svg(st);
//        let corner_path = Path::from_svg(SVG_CORNER).expect("Error loading SVG");
//        let side_path = Path::from_svg(SVG_SIDE).expect("Error loading SVG");
        let corner_path = Path::new();
        let side_path = Path::new();

        AppState {
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
            target: Point::new(82.0, 13.1),
            corner_path,
            side_path,
        }
    }

    pub fn show_all_info(&self) -> bool {
        self.show_labels
    }
}