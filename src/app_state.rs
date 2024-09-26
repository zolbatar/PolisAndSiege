use std::collections::HashMap;
use raylib::{RaylibHandle};
use raylib::prelude::Camera2D;
use crate::model::territory::Territory;

pub(crate) const MIN_ZOOM: f32 = 4.675;

pub struct AppState {
    pub width: i32,
    pub height: i32,
    pub half_width: i32,
    pub half_height: i32,
    pub dpi: f32,
    pub territories: HashMap<String, Territory>,
    pub camera: Camera2D,
    pub panning: bool,
}

impl AppState {
    pub fn new(rl: &RaylibHandle, territories: HashMap<String, Territory>) -> Self {
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        let half_width = width / 2;
        let half_height = height / 2;
        let dpi = rl.get_window_scale_dpi().x;
        let mut camera = Camera2D::default();
        camera.target.x = 23.0;
        camera.target.y = -9.5;
        camera.offset.x = half_width as f32;
        camera.offset.y = half_height as f32;
        camera.zoom = MIN_ZOOM;

        AppState {
            width,
            height,
            half_width,
            half_height,
            dpi,
            territories,
            camera,
            panning: false,
        }
    }

    pub fn show_all_info(&self) -> bool {
        true
        //        self.camera.zoom > 7.5
    }
}