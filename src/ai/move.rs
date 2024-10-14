use crate::ai::game_state::GameState;
use crate::app_state::AppState;
use crate::model::city_state::CityState;
use crate::model::player::Player;
use crate::next_turn;
use specs::WorldExt;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum MoveType {
    PlaceArmy,
}

pub struct Move {
    pub move_type: MoveType,
    pub city_state_source: Option<Arc<Mutex<CityState>>>,
    pub _city_state_target: Option<Arc<Mutex<CityState>>>,
    pub game_state: Option<GameState>,
    pub child_moves: Vec<Move>,
    pub best_score: i32,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We choose not to display `secret_field`
        f.debug_struct("Move")
            .field("move_type", &self.move_type)
            .field("best_score", &self.best_score)
/*            .field("score", &self.game_state.clone().unwrap().score)*/
            .field("child_moves", &self.child_moves)
            .finish()
    }
}

impl Move {
    pub fn new_place_army(
        app_state: &AppState,
        mut game_state: GameState,
        city_state_source: Arc<Mutex<CityState>>,
    ) -> Self {
        city_state_source.lock().unwrap().armies += 1;
        game_state.calculate_score(app_state);

        Self {
            move_type: MoveType::PlaceArmy,
            city_state_source: Some(city_state_source),
            _city_state_target: None,
            game_state: Some(game_state),
            child_moves: Vec::new(),
            best_score: 0,
        }
    }

    fn find_city(&self, app_state: &AppState, city_state_in: Arc<Mutex<CityState>>) -> Arc<Mutex<CityState>> {
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
                    let city = self.find_city(app_state, self.city_state_source.clone().unwrap());
                    city.lock().unwrap().armies += 1;
                    current_player.armies_to_assign -= 1;
                }
            }
        }
        next_turn(app_state);
    }
}
