use crate::ai::possible_move::possible_moves;
use crate::app_state::AppState;
use std::process::exit;

pub fn computer_turn(app_state: &mut AppState) {
    // Clone the world state totally
    let world_state = app_state.world_state.clone();

    // Get current player
    let player = app_state.world_state.current_player.as_ref().unwrap();
    print!("Starting score: {}, ", player.lock().unwrap().score);

    // Create initial game state
    let depth = 0;

    let mut possibles = possible_moves(&world_state, &app_state.world_fixed, depth);
    if possibles.is_empty() {
        println!("no possible moves");
        exit(0);
    } else {
        print!("there are {} possible moves, ", possibles.len());
    }

    // Score range
    let lowest = possibles.iter().min_by_key(|p| p.best_score).unwrap().best_score;
    let highest = possibles.iter().max_by_key(|p| p.best_score).unwrap().best_score;
    println!("lowest and highest score: {}/{}", lowest, highest);

    // Select move
    possibles.sort_by(|a, b| a.best_score.cmp(&b.best_score));
    let best = &mut possibles[0];
    best.do_move_and_next_turn(app_state);
    //    println!("{:#?}", possibles);
}
