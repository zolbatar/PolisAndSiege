use crate::ai::game_state::GameState;
use crate::ai::r#move::Move;
use crate::app_state::AppState;

pub fn ap_build_list_of_possibles(game_state: &GameState, app_state: &AppState) -> Vec<Move> {
    let mut results = Vec::new();
    let player_cities = &mut game_state.get_player_cities();
    for city in player_cities {
        let new_state = game_state.full_clone();
        let new_city = new_state.find_city(city.clone());
        let the_move = Move::new_place_army(app_state, new_state, new_city);
        results.push(the_move);
    }
    results
}
