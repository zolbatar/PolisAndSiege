use crate::app_state::{AppState, GameMode};
use crate::model::city::SIZE;
use crate::next_turn;
use sdl2::mouse::{MouseButton, MouseWheelDirection};
use skia_safe::Point;
use std::sync::Arc;

const THRESHOLD: i32 = 64;

pub fn handle_mouse_wheel(app_state: &mut AppState, _direction: MouseWheelDirection, precise_y: f32) {
    let delta = precise_y * 0.25;
    app_state.zoom += delta;
    app_state.zoom = app_state.zoom.clamp(crate::app_state::MIN_ZOOM, 15.0);
}

pub fn handle_mouse_motion(app_state: &mut AppState, x: i32, y: i32, x_rel: i32, y_rel: i32) {
    let world_state = &app_state.world_state;
    app_state.hover = Point::new(x as f32, y as f32);
    if app_state.panning {
        // Calculate mouse movement delta
        if x_rel.abs() < THRESHOLD && y_rel.abs() < THRESHOLD {
            // Update camera target based on mouse movement
            app_state.target.x -= x_rel as f32 / app_state.zoom;
            app_state.target.y -= y_rel as f32 / app_state.zoom;
        }
    } else {
        // Mouse over?
        if world_state.current_player.is_none() {
            return;
        }
        let current_player = &world_state.current_player.clone().unwrap();
        let mut mp = Point::new(x as f32, y as f32);

        // Do reverse matrix transform
        mp.x -= app_state.gfx.half_width as f32;
        mp.y -= app_state.gfx.half_height as f32;
        mp.x /= app_state.zoom;
        mp.y /= app_state.zoom;
        mp.x += app_state.target.x;
        mp.y += app_state.target.y;

        if world_state.mode == GameMode::ArmyPlacement {
            app_state.selection.last_city_selection = None;
        } else {
            app_state.selection.last_city_hover = None;
        }
        for _city_state in &world_state.city_states {
            let city_state = _city_state.lock().unwrap();
            let city = city_state.city.clone();
            if city_state.owner.is_some() {
                let owner = &city_state.owner.clone().unwrap();
                let delta = city.lock().unwrap().location.p - mp;
                let diff = (delta.x * delta.x + delta.y * delta.y).sqrt();
                if diff <= SIZE * app_state.zoom / app_state.gfx.dpi / 2.0 {
                    if Arc::ptr_eq(owner, current_player) {
                        if world_state.mode == GameMode::ArmyPlacement {
                            app_state.selection.last_city_selection = Some(_city_state.clone());
                        } else {
                            app_state.selection.last_city_hover = Some(_city_state.clone());
                        }
                    }
                }
            }
        }
    }
}

pub fn handle_mouse_button_down(app_state: &mut AppState, button: MouseButton) {
    let player = app_state.world_state.current_player.as_ref();
    let is_human = if app_state.world_state.current_player.is_none() {
        false
    } else {
        app_state.world_state.current_player.as_ref().unwrap().lock().unwrap().is_human()
    };
    if button == MouseButton::Right {
        app_state.panning = true;
    } else if button == MouseButton::Left {
        match app_state.world_state.mode {
            GameMode::ArmyPlacement => {
                if is_human && app_state.selection.last_city_selection.is_some() {
                    let city_state = app_state.selection.last_city_selection.clone();
                    let owner = city_state.as_ref().unwrap().lock().unwrap().owner.clone();
                    if owner.unwrap().lock().unwrap().is_human() {
                        city_state.unwrap().lock().unwrap().armies += 1;
                        player.unwrap().lock().unwrap().armies_to_assign -= 1;
                        if player.unwrap().lock().unwrap().armies_to_assign == 0 {
                            app_state.selection.last_city_hover = app_state.selection.last_city_selection.clone();
                            app_state.selection.last_city_selection = None;
                        }
                    }
                    next_turn(app_state);
                }
            }
            GameMode::Game => {
                app_state.selection.last_city_selection = app_state.selection.last_city_hover.clone();
            }
            _ => {}
        }
    }
}

pub fn handle_mouse_button_up(app_state: &mut AppState, button: MouseButton) {
    if button == MouseButton::Right {
        app_state.panning = false;
    }
}
