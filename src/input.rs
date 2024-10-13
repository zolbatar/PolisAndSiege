use crate::app_state::{AppState, GameMode};
use crate::model::city::{City, SIZE};
use crate::model::territory::Territory;
use sdl2::mouse::{MouseButton, MouseWheelDirection};
use skia_safe::Point;
use specs::{WorldExt};
use crate::model::city_state::CityState;
use crate::model::player::{Player};
use crate::next_turn;

const THRESHOLD: i32 = 64;

pub fn handle_mouse_wheel(app_state: &mut AppState, _direction: MouseWheelDirection, precise_y: f32) {
    let delta = precise_y * 0.25;
    app_state.zoom += delta;
    app_state.zoom = app_state.zoom.clamp(crate::app_state::MIN_ZOOM, 15.0);
}

pub fn handle_mouse_motion(app_state: &mut AppState, x: i32, y: i32, x_rel: i32, y_rel: i32) {
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
        let mut mp = Point::new(x as f32, y as f32);

        // Do reverse matrix transform
        mp.x -= app_state.gfx.half_width as f32;
        mp.y -= app_state.gfx.half_height as f32;
        mp.x /= app_state.zoom;
        mp.y /= app_state.zoom;
        mp.x += app_state.target.x;
        mp.y += app_state.target.y;

        if app_state.mode == GameMode::ArmyPlacement {
            app_state.selection.last_city_selection = None;
        } else {
            app_state.selection.last_city_hover = None;
        }
        let territories = app_state.world.read_storage::<Territory>();
        let mut cities = app_state.world.write_storage::<City>();
        let mut city_states = app_state.world.write_storage::<CityState>();
        for territory_entity in &app_state.items.territories {
            let territory = territories.get(*territory_entity.1).unwrap();
            for city_entity in territory.cities.iter() {
                let city_state = city_states.get_mut(*city_entity).unwrap();
                let city = cities.get_mut(city_state.city).unwrap();
                let delta = city.location.p - mp;
                let diff = (delta.x * delta.x + delta.y * delta.y).sqrt();
                if diff <= SIZE * app_state.zoom / app_state.gfx.dpi / 2.0 && city_state.owner.is_some() && city_state.owner.unwrap() == app_state.current_player {
                    if app_state.mode == GameMode::ArmyPlacement {
                        app_state.selection.last_city_selection = Some(*city_entity);
                    } else {
                        app_state.selection.last_city_hover = Some(*city_entity);
                    }
                }
            }
        }
    }
}

pub fn handle_mouse_button_down(app_state: &mut AppState, button: MouseButton) {
    let human_turn = app_state.current_player == app_state.actual_human;
    if button == MouseButton::Right {
        app_state.panning = true;
    } else if button == MouseButton::Left {
        match app_state.mode {
            GameMode::ArmyPlacement => {
                if human_turn {
                    if let Some(city_entity) = &app_state.selection.last_city_selection {
                        let mut city_states = app_state.world.write_storage::<CityState>();
                        let mut players = app_state.world.write_storage::<Player>();
                        let city_state = city_states.get_mut(*city_entity).unwrap();
                        let player = players.get_mut(app_state.current_player).unwrap();
                        if city_state.owner.unwrap() == app_state.current_player {
                            city_state.armies += 1;
                            player.armies_to_assign -= 1;
                            if player.armies_to_assign == 0 {
                                app_state.selection.last_city_hover = app_state.selection.last_city_selection;
                                app_state.selection.last_city_selection = None;
                            }
                        }
                    }
                    next_turn(app_state);
                }
            }
            GameMode::Game => {
                app_state.selection.last_city_selection = app_state.selection.last_city_hover;
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
