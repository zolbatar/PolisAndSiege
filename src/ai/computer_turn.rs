use crate::app_state::AppState;
use crate::game_state::GameState;
use crate::ai::possible_move::possible_moves;

pub fn computer_turn(app_state: &mut AppState) {
    // Create initial game state
    let game_state = GameState {
        actual_human: Some(app_state.actual_human),
        current_turn: Some(app_state.current_turn),
        players: app_state.players.clone(),
        territories: app_state.items.territories.clone(),
        mode: app_state.mode.clone(),
    };

    let possibles = possible_moves(&game_state);
    if possibles.is_empty() { 
        panic!("No possible moves");
    } 
}
