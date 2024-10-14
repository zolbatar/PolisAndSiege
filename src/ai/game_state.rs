use crate::ai::temp_player::TempPlayer;
use crate::app_state::{AppState, GameMode};
use crate::model::city::City;
use crate::model::city_state::CityState;
use crate::model::player::score_for_city;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use specs::{Entity, WorldExt};
use std::sync::{Arc, Mutex};

#[derive(Default, Clone, Debug)]
pub struct GameState {
    pub score: i32,
    pub current_player: Option<Entity>,
    pub players: Vec<TempPlayer>,
    pub city_states: Vec<Arc<Mutex<CityState>>>,
    pub mode: GameMode,
    pub depth: u8,
    pub requested_depth: u8,
    pub no_choices: usize,
}

impl GameState {
    pub fn full_clone(&self) -> GameState {
        let mut new_state = self.clone();
        new_state.city_states.clear();
        for city_state in self.city_states.iter() {
            let city_state_unwrapped = city_state.lock().unwrap();
            new_state.city_states.push(Arc::new(Mutex::new(CityState {
                city: city_state_unwrapped.city,
                armies: city_state_unwrapped.armies,
                owner: city_state_unwrapped.owner,
            })));
        }
        new_state
    }
    pub fn get_player_cities(&self) -> Vec<Arc<Mutex<CityState>>> {
        let mut cities = Vec::new();
        for city in self.city_states.iter() {
            if city.lock().unwrap().owner == self.current_player {
                cities.push(city.clone());
            }
        }

        // Shuffle (for now)
        let mut rng = thread_rng(); // Create a random number generator
        cities.shuffle(&mut rng);

        cities
    }

    pub fn display_score(&self) {
        println!("Score: {}", self.score);
    }

    pub fn calculate_score(&mut self, app_state: &AppState) {
        let cities = app_state.world.read_storage::<City>();
        self.score = 0;
        let player_cities = self.get_player_cities();
        for city in player_cities {
            let city_entity = cities.get(city.lock().unwrap().city);
            self.score += score_for_city(city_entity.unwrap(), &city.lock().unwrap());
        }
        
        // Add randomness to keep it interesting and less easy to guess intentions
        let range = self.score / 15;
        let mut r = thread_rng();
        self.score += r.gen_range(0..range);
    }

    pub fn find_city(&self, city_state_in: Arc<Mutex<CityState>>) -> Arc<Mutex<CityState>> {
        let search_entity = city_state_in.lock().unwrap().city;
        for city_state in &self.get_player_cities() {
            if city_state.lock().unwrap().city == search_entity {
                return city_state.clone();
            }
        }
        panic!("find_city failed");
    }
}
