use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
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
    Randomising,
    CitySelection,
    Game,
}

pub struct GFXState {
    pub width: i32,
    pub height: i32,
    pub half_width: i32,
    pub half_height: i32,
    pub dpi: f32,
}

pub struct Resource {
    pub side_path: Dom,
    pub corner_path: Dom,
}

pub struct Items {
    pub territories: HashMap<String, Arc<Mutex<Territory>>>,
    pub existing_cities: Vec<Location>,
    pub cities: Vec<Arc<Mutex<City>>>,
    pub cities_remaining_to_assign: Vec<Arc<Mutex<City>>>,
    pub connections: Vec<Connection>,
}

pub struct CitySelection {
    pub last_selection: Instant,
    pub last_city_selection: Option<Arc<Mutex<City>>>,
}

pub struct AppState {
    pub selection: CitySelection,
    pub gfx: GFXState,
    pub res: Resource,
    pub items: Items,
    pub mode: GameMode,
    pub num_of_players: u8,
    pub fps: f64,
    pub zoom: f32,
    pub target: Point,
    pub panning: bool,
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

        let gfx = GFXState {
            width,
            height,
            half_width,
            half_height,
            dpi,
        };

        let res = Resource {
            corner_path,
            side_path,
        };

        let items = Items {
            territories: HashMap::new(),
            existing_cities: Vec::new(),
            connections: Vec::new(),
            cities: Vec::new(),
            cities_remaining_to_assign: Vec::new(),
        };

        AppState {
            mode: GameMode::Randomising,
            selection: CitySelection {
                last_selection: Instant::now(),
                last_city_selection: None,
            },
            gfx,
            res,
            items,
            num_of_players: 2,
            fps: 0.0,
            panning: false,
            show_labels: true,
            show_shadows: true,
            zoom: MIN_ZOOM,
            target: Point::new(25.0, -10.0),
        }
    }

    pub fn show_all_info(&self) -> bool {
        self.show_labels
    }
}