use crate::ai::possible_move::possible_moves;
use crate::app_state::{AppState, GameMode};
use crate::next_turn;

pub fn computer_turn_by_phase(app_state: &mut AppState, mode: GameMode) {
    let mut possibles = possible_moves(&app_state.world_state, &mut app_state.world_fixed, 0, mode.clone());
    if mode == GameMode::Game {
        println!("Attacks: {}", possibles.len());
    }

    if possibles.is_empty() {
        println!("no possible {:?} moves", mode);
        //app_state.world_state.mode = GameMode::End;
        return;
    } else {
        print!("there are {} possible {:?} moves, ", possibles.len(), mode);
    }

    // Score range
    let lowest = possibles.iter().min_by_key(|p| p.score_portion).unwrap().score_portion;
    let highest = possibles.iter().max_by_key(|p| p.score_portion).unwrap().score_portion;
    println!("lowest and highest score: {}/{}", lowest, highest);

    // Select move(s)
    possibles.sort_by(|a, b| a.score_portion.cmp(&b.score_portion));
    //    println!("{:#?}", possibles);

    let world_state = &mut app_state.world_state;
    match mode {
        GameMode::ArmyPlacement => {
            while !possibles.is_empty() && world_state.current_player.as_ref().unwrap().borrow().armies_to_assign > 0 {
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
}

pub fn computer_turn(app_state: &mut AppState) {
    // Get current player
    let player = app_state.world_state.current_player.as_ref().unwrap();
    print!("Starting score: {}, {} armies to place, ", player.borrow().score, player.borrow().armies_to_assign);
    if player.borrow().armies_to_assign > 0 {
        computer_turn_by_phase(app_state, GameMode::ArmyPlacement);
    }
    computer_turn_by_phase(app_state, GameMode::Game);

    next_turn(app_state);
}
