use crate::app_state::AppState;
use crate::ai::game_state::GameState;
use crate::ai::possible_move::possible_moves;

pub fn computer_turn(app_state: &mut AppState) {

    // Create initial game state
    let game_state = GameState {
        actual_human: Some(app_state.actual_human),
        current_turn: Some(app_state.current_player),
        players: app_state.players.clone(),
        territories: app_state.items.territories.clone(),
        mode: app_state.mode.clone(),
        depth: 0,
        requested_depth: 0,
    };

    let mut possibles = possible_moves(&game_state, app_state);
    if possibles.is_empty() {
        println!("No possible moves");
        return;
    }

    // Select move
    possibles.sort_by(|a, b| a.score.cmp(&b.score));
    let best = &possibles[0];
    best.do_move_and_next_turn(app_state);
    //    println!("{:?}", possibles);
}
