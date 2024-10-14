use crate::ai::game_state::GameState;
use crate::ai::r#move::Move;
use crate::app_state::{AppState, GameMode};
use crate::model::player::Player;
use specs::WorldExt;

pub fn possible_moves(game_state: &GameState, app_state: &AppState) -> Vec<Move> {
    let mut results = Vec::new();
    let players = app_state.world.read_storage::<Player>();
    let player_cities = &mut game_state.get_player_cities();

    match game_state.mode {
        GameMode::ArmyPlacement => {

            // Build an initial list of possibles
            for city in player_cities {
                let new_state = game_state.full_clone();
                let new_city = new_state.find_city(city.clone());
                let the_move = Move::new_place_army(app_state, new_state, new_city);
                results.push(the_move);
            }

            // Slice down to 3?
            results.sort_by(|a, b| {
                a.game_state.as_ref().map(|gs| gs.score)
                    .partial_cmp(&b.game_state.as_ref().map(|gs| gs.score))
                    .unwrap_or(std::cmp::Ordering::Equal).reverse()
            });
            results = results.into_iter().take(game_state.no_choices).collect();

            // Go deeper?
            if game_state.depth != game_state.requested_depth {
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
                    let additional_moves = &mut possible_moves(new_state, app_state);

                    result.child_moves.append(additional_moves);
                }
            }
        }
        GameMode::Randomising => {}
        GameMode::Game => {}
    }

    // Set best score
    for result in &mut results {
        result.best_score = result.game_state.as_ref().unwrap().score;
        let top = result.child_moves.iter().max_by_key(|p| p.game_state.as_ref().unwrap()
            .score);
        if let Some(top) = top {
            let highest = top.game_state.as_ref().unwrap().score;
            result.best_score = highest;
        }
    }

    //    println!("Returning: {}", results.len());
    results
}
