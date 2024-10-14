use crate::ai::game_state::GameState;
use crate::app_state::AppState;
use crate::model::city::City;
use crate::model::city_state::CityState;
use crate::model::player::Player;
use crate::next_turn;
use specs::WorldExt;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum MoveType {
    PlaceArmy,
}

#[derive(Debug)]
pub struct Move {
    move_type: MoveType,
    city_state_source: Arc<Mutex<CityState>>,
    city_state_target: Option<Arc<Mutex<CityState>>>,
    pub game_state: GameState,
}

impl Move {
    pub fn new_place_army(
        app_state: &AppState,
        mut game_state: GameState,
        city_state_source: Arc<Mutex<CityState>>,
    ) -> Self {
        city_state_source.lock().unwrap().armies += 1;
        let cities = app_state.world.read_storage::<City>();
        let city_entity = city_state_source.lock().unwrap().city;
        //println!("Move: army place for {}", cities.get(city_entity).unwrap().name);
        game_state.calculate_score(app_state);

        Self {
            move_type: MoveType::PlaceArmy,
            city_state_source,
            city_state_target: None,
            game_state,
        }
    }

    pub fn find_city(&self, app_state: &AppState, city_state_in: Arc<Mutex<CityState>>) -> Arc<Mutex<CityState>> {
        let search_entity = city_state_in.lock().unwrap().city;
        for city_state_entity in &app_state.items.cities {
            if city_state_entity.lock().unwrap().city == search_entity {
                return city_state_entity.clone();
            }
        }
        panic!("find_city failed");
    }

    pub fn do_move_and_next_turn(&mut self, app_state: &mut AppState) {
        {
            let mut players = app_state.world.write_storage::<Player>();
            let current_player = players.get_mut(app_state.current_player).unwrap();
            match self.move_type {
                MoveType::PlaceArmy => {
                    let city = self.find_city(app_state, self.city_state_source.clone());
                    city.lock().unwrap().armies += 1;
                    current_player.armies_to_assign -= 1;
                }
            }
        }
        next_turn(app_state);
    }
}
