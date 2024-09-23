use raylib::{RaylibHandle, RaylibThread};

pub struct AppState {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub width: i32,
    pub height: i32,
    pub half_width: i32,
    pub half_height: i32,
    pub dpi: f32,
}

// You can also define associated methods for the struct if needed
impl AppState {
    pub fn new(rl: RaylibHandle, thread: RaylibThread) -> Self {
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        let half_width = width / 2;
        let half_height = height / 2;
        let dpi = rl.get_window_scale_dpi().x;

        AppState {
            rl,
            thread,
            width,
            height,
            half_width,
            half_height,
            dpi,
        }
    }
}