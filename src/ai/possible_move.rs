use crate::ai::army_placement::ap_build_list_of_possibles;
use crate::ai::game::game_build_list_of_possibles;
use crate::ai::moves::{Move, MoveType};
use crate::app_state::GameMode;
use crate::model::world_fixed::WorldFixed;
use crate::model::world_state::WorldState;

fn reduce_down_to_limited_list(data_in: Vec<Move>) -> Vec<Move> {
    let mut results = data_in;
    results.sort_by(|a, b| a.score_portion.partial_cmp(&b.score_portion).unwrap().reverse());
    results
}

fn go_deeper<F>(
    world_state: &WorldState,
    world_fixed: &WorldFixed,
    data_in: Vec<Move>,
    mut f: F,
    depth: usize,
) -> Vec<Move>
where
    F: FnMut(&WorldState, usize) -> Vec<Move>,
{
    let mut results = data_in;
    let desired_depth = world_state.current_player.as_ref().unwrap().borrow().profile.search_depth;
    if depth != desired_depth {
        for result in &mut results {
            let mut world_state = result.world_state.deep_clone();

            // Update player
            {
                let player_index = world_state.current_player.as_ref().unwrap().borrow().index;
                let new_player = &mut world_state.players[player_index];

                // Phase done
                match &world_state.mode {
                    GameMode::ArmyPlacement => {
                        new_player.borrow_mut().armies_to_assign -= 1;
                        if new_player.borrow().armies_to_assign == 0 {
                            world_state.mode = GameMode::Game;
                        }
                    }
                    GameMode::Game => {}
                    _ => panic!("Unknown mode"),
                }
            }

            // Next player and pretend again
            /*                    loop {
                let mut other_player_state = new_state.clone();
                if move_to_next_player(&mut other_player_state, app_state, player_index) {
                    break;
                }
            }*/

            // Recurse
            let mut additional_moves = f(&world_state, depth + 1);
            result.child_moves.append(&mut additional_moves);
        }
    }
    results
}

pub fn possible_moves(
    world_state: &WorldState,
    world_fixed: &mut WorldFixed,
    depth: usize,
    mode: GameMode,
) -> Vec<Move> {
    let mut results: Vec<Move> = Vec::new();

    // Build a list of all possible moves
    let world_state = world_state.deep_clone();
    let current_player = world_state.get_current_player();
    match mode {
        GameMode::Randomising => panic!("This should not happen"),
        GameMode::ArmyPlacement => {
            if current_player.borrow().armies_to_assign == 0 {
                return results;
            }
            results = ap_build_list_of_possibles(&world_state, current_player.borrow().index);
        }
        GameMode::Game => results = game_build_list_of_possibles(&world_state, current_player.borrow().index),
        _ => {}
    }

    // Now do each of the moves and work out the scores
    for result in &mut results {
        let mut world_state = world_state.deep_clone();
        let current_player = world_state.get_current_player();

        // If this is an attack, work out combat delta
        let attack_delta = if result.move_type == MoveType::AttackCity {
            let source_armies = world_state.cities[result.city_source.unwrap()].borrow().armies as f32;
            let target_armies = world_state.cities[result.city_target.unwrap()].borrow().armies as f32;
            source_armies - target_armies
        } else {
            0f32
        };

        result.do_move(&mut world_state);
        world_state.update_scores(world_fixed);
        let all_scores: i32 = world_state.players.iter().map(|p| p.borrow().score).sum();
        let mut current_player_score = world_state.current_player.as_ref().unwrap().borrow().score;

        // If this is an attack, encourage it
        if result.move_type == MoveType::AttackCity {
            current_player_score += (attack_delta * current_player.borrow().profile.attack_delta_multiplier) as i32;
        }

        result.score_portion = (current_player_score * 10000) / all_scores;
        result.world_state = world_state;
    }

    // Select x of the list
    results = reduce_down_to_limited_list(results);

    // Go deeper if required
    //    results = go_deeper(&world_state, results, possible_moves, depth);

    results
}
