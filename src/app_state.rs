use crate::skia::{MySurface, Skia};
use raylib::color::Color;
use raylib::ffi::{DrawTexturePro, GetCurrentMonitor, GetMonitorHeight, GetMonitorWidth, GetScreenHeight, GetScreenWidth, Vector2};
use raylib::prelude::RaylibDraw;
use raylib::{RaylibHandle, RaylibThread};
use skia_safe::Canvas;

pub struct AppState {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub width: i32,
    pub height: i32,
    //    pub half_width: i32,
    //    pub half_height: i32,
    pub dpi: f32,
    pub skia: Skia,
    pub surface: MySurface,
}

impl AppState {
    pub fn new(rl: RaylibHandle, thread: RaylibThread) -> Self {
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        //        let half_width = width / 2;
        //        let half_height = height / 2;
        let dpi = rl.get_window_scale_dpi().x;
        unsafe {
            let monitor = GetCurrentMonitor();
            println!("Native resolution: {} x {} ({} DPI)", GetMonitorWidth(monitor), GetMonitorHeight(monitor), dpi);
        }
        println!("Window resolution: {} x {}", width, height);
        let mut skia = Skia::new();
        let surface = skia.make_surface(width * dpi as i32, height * dpi as i32);

        AppState {
            rl,
            thread,
            width,
            height,
            //            half_width,
            //            half_height,
            dpi,
            skia,
            surface,
        }
    }

    pub unsafe fn render(&mut self) {
        unsafe { self.skia.flush(&mut self.surface); }

        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        let source = raylib::ffi::Rectangle { x: 0.0, y: 0.0, width: (self.width as f32) * self.dpi, height: (self.height as f32) * self.dpi };
        let dest = raylib::ffi::Rectangle { x: 0.0, y: 0.0, width: (self.width as f32), height: self.height as f32 };
        DrawTexturePro(
            self.surface.texture.texture,
            source, dest,
            Vector2 { x: 0.0, y: 0.0 }, 0.0,
            raylib::ffi::Color { a: 255, r: 255, g: 255, b: 255 });
    }
}