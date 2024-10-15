use crate::ai::army_placement::ap_build_list_of_possibles;
use crate::ai::game_state::GameState;
use crate::ai::r#move::Move;
use crate::app_state::{AppState, GameMode};
use crate::model::player::Player;
use specs::WorldExt;

fn reduce_down_to_limited_list(game_state: &GameState, data_in: Vec<Move>) -> Vec<Move> {
    let mut results = data_in;
    results.sort_by(|a, b| {
        a.game_state
            .as_ref()
            .map(|gs| gs.score)
            .partial_cmp(&b.game_state.as_ref().map(|gs| gs.score))
            .unwrap_or(std::cmp::Ordering::Equal)
            .reverse()
    });
    results.into_iter().take(game_state.no_choices).collect()
}

fn go_deeper<F>(game_state: &GameState, app_state: &AppState, data_in: Vec<Move>, mut f: F) -> Vec<Move>
where
    F: FnMut(&GameState, &AppState) -> Vec<Move>,
{
    let mut results = data_in;
    let players = app_state.world.read_storage::<Player>();
    if game_state.depth != players.get(game_state.current_player.unwrap()).unwrap().profile.search_depth {
        for result in &mut results {
            let new_state = &mut result.game_state.clone().unwrap();
            new_state.depth += 1;

            // Update player
            let player_index = players.get(new_state.current_player.unwrap()).unwrap().index;
            new_state.players[player_index].armies_to_assign -= 1;

            // Phase done
            match &new_state.mode {
                GameMode::ArmyPlacement => {
                    if new_state.players[player_index].armies_to_assign == 0 {
                        new_state.mode = GameMode::Game;
                    }
                }
                _ => panic!("Unknown mode"),
            }

            // Next player and pretend again
            /*                    loop {
                let mut other_player_state = new_state.clone();
                if move_to_next_player(&mut other_player_state, app_state, player_index) {
                    break;
                }
            }*/

            // Recurse
            let mut additional_moves = f(new_state, app_state);
            result.child_moves.append(&mut additional_moves);
        }
    }
    results
}

pub fn possible_moves(game_state: &GameState, app_state: &AppState) -> Vec<Move> {
    let mut results = Vec::new();

    match game_state.mode {
        GameMode::ArmyPlacement => {
            results = ap_build_list_of_possibles(game_state, app_state);
            results = reduce_down_to_limited_list(game_state, results);
            results = go_deeper(game_state, app_state, results, possible_moves);
        }
        GameMode::Randomising => {}
        GameMode::Game => {}
    }

    // Set best score
    for result in &mut results {
        result.best_score = result.game_state.as_ref().unwrap().score;
        let top = result.child_moves.iter().max_by_key(|p| p.game_state.as_ref().unwrap().score);
        if let Some(top) = top {
            let highest = top.game_state.as_ref().unwrap().score;
            result.best_score = highest;
        }
    }

    results
}
