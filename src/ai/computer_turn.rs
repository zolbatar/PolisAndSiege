use crate::ai::possible_move::possible_moves;
use crate::app_state::{AppState, GameMode};
use crate::next_turn;

pub fn computer_turn(app_state: &mut AppState) {
    // Get current player
    let player = app_state.world_state.current_player.as_ref().unwrap();
    print!("Starting score: {}, ", player.borrow().score);

    // Create initial game state
    let depth = 0;

    let mut possibles = possible_moves(&app_state.world_state, &mut app_state.world_fixed, depth);
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

    // Select move(s)
    possibles.sort_by(|a, b| a.score_portion.cmp(&b.score_portion));
    for possible in &possibles {
        println!("Score: {}", possible.score_portion);
    }
//    println!("{:#?}", possibles);

    let world_state = &mut app_state.world_state;
    match world_state.mode {
        GameMode::ArmyPlacement => {
            while !possibles.is_empty() && world_state.current_player.as_ref().unwrap().borrow()
                .armies_to_assign > 0 {
                let the_move = possibles.pop().unwrap();
                the_move.do_move(world_state);
            }
        }
        GameMode::Game => {
            for the_move in possibles {
                the_move.do_move(world_state);
            }
        }
        _ => {}
    }

    next_turn(app_state);
}
