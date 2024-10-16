use crate::ai::army_placement::ap_build_list_of_possibles;
use crate::ai::moves::Move;
use crate::app_state::GameMode;
use crate::model::ai_profile::AIProfile;
use crate::model::world_fixed::WorldFixed;
use crate::model::world_state::WorldState;

fn reduce_down_to_limited_list(profile: &AIProfile, data_in: Vec<Move>) -> Vec<Move> {
    let mut results = data_in;
    results.sort_by(|a, b| a.best_score.partial_cmp(&b.best_score).unwrap().reverse());
    results.into_iter().take(profile.no_choices).collect()
}

fn go_deeper<F>(
    world_state: &WorldState,
    world_fixed: &WorldFixed,
    data_in: Vec<Move>,
    mut f: F,
    depth: usize,
) -> Vec<Move>
where
    F: FnMut(&WorldState, &WorldFixed, usize) -> Vec<Move>,
{
    let mut results = data_in;
    let desired_depth = world_state.current_player.as_ref().unwrap().lock().unwrap().profile.search_depth;
    if depth != desired_depth {
        for result in &mut results {
            let mut world_state = world_state.deep_clone();

            // Update player
            {
                let player_index = world_state.current_player.as_ref().unwrap().lock().unwrap().index;
                let new_player = &mut world_state.players[player_index].lock().unwrap();
                new_player.armies_to_assign -= 1;

                // Phase done
                match &world_state.mode {
                    GameMode::ArmyPlacement => {
                        if new_player.armies_to_assign == 0 {
                            world_state.mode = GameMode::Game;
                        }
                    }
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
            let mut additional_moves = f(&world_state, world_fixed, depth + 1);
            result.child_moves.append(&mut additional_moves);
        }
    }
    results
}

pub fn possible_moves(world_state: &WorldState, world_fixed: &WorldFixed, depth: usize) -> Vec<Move> {
    let mut results: Vec<Move> = Vec::new();
    let world_state = world_state.deep_clone();

    match world_state.mode {
        GameMode::ArmyPlacement => {
            results = ap_build_list_of_possibles(&world_state);

            // Update scores
            {
                let current_player = world_state.current_player.as_ref().unwrap();
                for result in &mut results {
                    result.best_score = current_player.lock().unwrap().get_score() as i32;
                }
            }

            // Select x of the list
            {
                let current_player = world_state.current_player.as_ref().unwrap();
                results = reduce_down_to_limited_list(&current_player.lock().unwrap().profile, results);
            }

            // Go deeper if required
            results = go_deeper(&world_state, world_fixed, results, possible_moves, depth);
        }
        GameMode::Randomising => {}
        GameMode::Game => {}
    }

    // Set best score
    for result in &mut results {
        let top = result.child_moves.iter().max_by_key(|p| p.best_score);
        if let Some(top) = top {
            let highest = top.best_score;
            result.best_score = highest;
        }
    }

    results
}
