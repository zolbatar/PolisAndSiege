use crate::ai::game_state::GameState;
use crate::ai::r#move::Move;
use crate::app_state::{AppState, GameMode};
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn possible_moves(game_state: &GameState, app_state: &AppState) -> Vec<Move> {
    let mut results = Vec::new();

    // Get player cities
    let player_cities = &mut game_state.get_player_cities();
    let mut rng = thread_rng(); // Create a random number generator
    player_cities.shuffle(&mut rng);

    match game_state.mode {
        GameMode::ArmyPlacement => {
            for city in player_cities {
                let mut new_state = game_state.full_clone();
                let new_city = new_state.find_city(city.clone());
                let the_move = Move::new_place_army(app_state, new_state.clone(), new_city);

                // Go deeper
                if new_state.depth != game_state.requested_depth {
                    new_state.depth += 1;
                    possible_moves(&new_state, app_state);
                }

                // If top level, add to results
                if game_state.depth == 0 {
                    results.push(the_move);
                }
            }
        }
        GameMode::Randomising => {}
        GameMode::Game => {}
    }

    results
}
