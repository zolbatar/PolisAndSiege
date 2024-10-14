use crate::ai::game_state::GameState;
use crate::ai::r#move::Move;
use crate::app_state::{AppState, GameMode};

pub fn possible_moves(game_state: &GameState, app_state: &AppState) -> Vec<Move> {
    let mut results = Vec::new();
    match game_state.mode {
        GameMode::ArmyPlacement => {
            for city in &game_state.get_player_cities() {
                let new_state = game_state.full_clone();
                let new_city = new_state.find_city(city.clone());
                let the_move = Move::new_place_army(app_state, new_state, new_city);
                results.push(the_move);
                if game_state.depth != game_state.requested_depth {
                    panic!("Invalid depth");
                    //let new_state = game_state.clone();
                    // Next depth
                    //return possible_moves(&new_state, app_state);
                }
            }
        }
        GameMode::Randomising => {}
        GameMode::Game => {}
    }

    results
}
