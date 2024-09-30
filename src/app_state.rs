use std::collections::HashMap;
use sdl2::video::Window;
use skia_safe::Point;
use crate::model::connection::Connection;
use crate::model::location::Location;
use crate::model::territory::Territory;

pub(crate) const MIN_ZOOM: f32 = 4.675;

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
}

impl AppState {
    pub fn new(window: &Window, dpi: f32) -> Self {
        let width = window.size().0 as i32;
        let height = window.size().1 as i32;
        let half_width = width / 2;
        let half_height = height / 2;

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
            target: Point::new(23.0, -9.5),
        }
    }

    pub fn show_all_info(&self) -> bool {
        self.show_labels
    }
}