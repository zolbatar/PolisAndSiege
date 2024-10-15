use crate::ai::game_state::GameState;
use crate::app_state::AppState;
use crate::model::city::City;
use crate::model::city_state::CityState;
use crate::model::player::Player;
use specs::{Entity, Join, ReadStorage, WorldExt};
use std::sync::{Arc, Mutex};

pub fn score_for_city(
    cities: &ReadStorage<City>,
    city: &City,
    city_state: CityState,
    other_connections: &Vec<Arc<Mutex<CityState>>>,
    player: &Player,
    player_entity: Entity,
) -> f32 {
    let mut score = 0f32;
    score += city.size as f32 * player.profile.city_size_multiplier;
    score += city_state.armies as f32 * player.profile.army_multiplier;

    // Logic for additional armies, extra score if bordering enemy concentrations
    for connection in other_connections {
        let other_city_entity = cities.get(connection.lock().unwrap().city).unwrap();
        let other_city_owner = connection.lock().unwrap().owner;

        // If enemy city, add a boost
        let boost = score;
        if other_city_owner.unwrap() != player_entity {
            score += city_state.additional_armies as f32 * player.profile.army_bordering;
            if other_city_entity.territory != city.territory {
                score += city_state.additional_armies as f32 * player.profile.army_same_territory;
            }
        }
        let boost_diff = score - boost;
        if boost_diff > 0.0 {
            println!("City {} has a boost of {}", city.name, boost_diff);
        }
    }

    score
}

pub fn game_state_scoring(game_state: &mut GameState, app_state: &AppState) {
    let players = app_state.world.read_storage::<Player>();
    for player in players.join() {
        //        player.update_score();
    }
}

pub fn app_state_scoring(app_state: &AppState) {
    let players = app_state.world.read_storage::<Player>();
    for player in players.join() {
        //        player.player.update_score();
    }
}
