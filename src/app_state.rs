use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use sdl2::video::Window;
use skia_safe::{FontMgr, Path, Point, Size};
use crate::model::connection::Connection;
use crate::model::location::Location;
use crate::model::territory::Territory;
use skia_safe::svg::Dom;
use crate::model::city::City;

const SVG_CORNER: &str = include_str!("../assets/Corner.svg");
const SVG_SIDE: &str = include_str!("../assets/Side.svg");
pub const NOISE_MIX: f32 = 0.075;
pub(crate) const MIN_ZOOM: f32 = 4.2;

pub enum GameMode {
    CitySelection,
    Game,
}

pub struct AppState {
    pub mode: GameMode,
    pub fps: f64,
    pub width: i32,
    pub height: i32,
    pub half_width: i32,
    pub half_height: i32,
    pub dpi: f32,
    pub zoom: f32,
    pub target: Point,
    pub panning: bool,
    pub territories: HashMap<String, Arc<Mutex<Territory>>>,
    pub existing_cities: Vec<Location>,
    pub cities: Vec<Arc<Mutex<City>>>,
    pub connections: Vec<Connection>,
    pub side_path: Dom,
    pub corner_path: Dom,

    pub show_labels: bool,
    pub show_shadows: bool,
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
            mode: GameMode::CitySelection,
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
            cities: Vec::new(),
            show_labels: true,
            show_shadows: true,
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