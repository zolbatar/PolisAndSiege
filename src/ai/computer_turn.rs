use crate::ai::possible_move::possible_moves;
use crate::app_state::AppState;
use crate::next_turn;

pub fn computer_turn(app_state: &mut AppState) {
    // Clone the world state totally
    let world_state = app_state.world_state.clone();

    // Get current player
    let player = app_state.world_state.current_player.as_ref().unwrap();
    print!("Starting score: {}, ", player.borrow().score);

    // Create initial game state
    let depth = 0;

    let mut possibles = possible_moves(&world_state, depth);
    if possibles.is_empty() {
        println!("no possible moves");
        //app_state.world_state.mode = GameMode::End;
        next_turn(app_state);
        return;
    } else {
        print!("there are {} possible moves, ", possibles.len());
    }

    // Score range
    let lowest = possibles.iter().min_by_key(|p| p.score_portion).unwrap().score_portion;
    let highest = possibles.iter().max_by_key(|p| p.score_portion).unwrap().score_portion;
    println!("lowest and highest score: {}/{}", lowest, highest);

    // Select move
    possibles.sort_by(|a, b| a.score_portion.cmp(&b.score_portion));
    let best = &possibles[0];
    best.do_move(&mut app_state.world_state, true);
    next_turn(app_state);
    //    println!("{:#?}", possibles);
}
