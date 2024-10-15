use crate::ai::game_state::GameState;
use crate::app_state::AppState;
use crate::model::city::City;
use crate::model::city_state::CityState;
use crate::model::connection::Connection;
use crate::model::player::Player;
use rand::{thread_rng, Rng};
use specs::{Entity, Join, ReadStorage, System, WorldExt, WriteStorage};
use std::sync::{Arc, Mutex};

pub const CITY_SIZE_MULTIPLIER: i32 = 3;
pub const ARMY_MULTIPLIER: i32 = 1;
pub const SCORE_ARMY_SAME_TERRITORY: i32 = 1;
pub const SCORE_ARMY_BORDERING: i32 = 2;
pub const RANDOM_DIVISOR: i32 = 15;

pub fn score_for_city(
    cities: &ReadStorage<City>,
    city: &City,
    city_state: CityState,
    other_connections: Option<Vec<Arc<Mutex<CityState>>>>,
    player: Option<Entity>,
) -> i32 {
    let mut score = 0;
    score += (city.size as i32) * CITY_SIZE_MULTIPLIER;
    score += city_state.armies as i32 * ARMY_MULTIPLIER;

    // Logic for additional armies, extra score if bordering enemy concentrations
    if let Some(other_connection) = other_connections {
        for connection in other_connection {
            let other_city_entity = cities.get(connection.lock().unwrap().city).unwrap();
            let other_city_owner = connection.lock().unwrap().owner;

            // If enemy city, add a boost
            let boost = score;
            if other_city_owner != player {
                score += city_state.additional_armies as i32 * SCORE_ARMY_BORDERING;
                if other_city_entity.territory != city.territory {
                    score += city_state.additional_armies as i32 * SCORE_ARMY_SAME_TERRITORY;
                }
            }
            let boost_diff = score - boost;
            if boost_diff > 0 {
                println!("City {} has a boost of {}", city.name, boost_diff);
            }
        }
    }

    score
}

pub fn game_state_scoring(game_state: &mut GameState, app_state: &AppState) {
    let connections = app_state.world.read_storage::<Connection>();
    let cities = app_state.world.read_storage::<City>();
    game_state.score = 0;
    let player_cities = game_state.get_player_cities();
    for city in &player_cities {
        let city_entity = cities.get(city.lock().unwrap().city);

        // Get connections
        let mut outgoing_connections = Vec::new();
        for connection_entity in &city_entity.unwrap().connections {
            let connection = connections.get(*connection_entity).unwrap();
            if connection.city1 == city.lock().unwrap().city {
                let outgoing_city_state =
                    game_state.city_states.iter().find(|city| city.lock().unwrap().city == connection.city2).unwrap();
                outgoing_connections.push(outgoing_city_state.clone());
            }
        }

        game_state.score += score_for_city(
            &app_state.world.read_storage::<City>(),
            city_entity.unwrap(),
            city.lock().unwrap().clone(),
            Some(outgoing_connections),
            game_state.current_player,
        );
    }

    // Add randomness to keep it interesting and less easy to guess intentions
    let range = game_state.score / RANDOM_DIVISOR;
    let mut r = thread_rng();
    game_state.score += r.gen_range(0..range);
}

pub struct SUpdateScores;
impl<'a> System<'a> for SUpdateScores {
    type SystemData = (WriteStorage<'a, Player>, ReadStorage<'a, City>);

    fn run(&mut self, (mut components, cities): Self::SystemData) {
        for component in (&mut components).join() {
            component.score = 0;
            for city_state in &component.cities {
                let city = cities.get(city_state.lock().unwrap().city);
                component.score +=
                    score_for_city(&cities, city.unwrap(), city_state.lock().unwrap().clone(), None, None);
            }
        }
    }
}
