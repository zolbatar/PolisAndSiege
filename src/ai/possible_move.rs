use crate::ai::computer_turn::move_to_next_player;
use crate::ai::game_state::GameState;
use crate::ai::r#move::Move;
use crate::app_state::{AppState, GameMode};
use crate::model::player::Player;
use specs::WorldExt;

pub fn possible_moves(game_state: &GameState, app_state: &AppState) -> Vec<Move> {
    println!("Depth: {}", game_state.depth);
    let mut results = Vec::new();

    // Get player cities
    let players = app_state.world.read_storage::<Player>();
    let player_cities = &mut game_state.get_player_cities();

    match game_state.mode {
        GameMode::ArmyPlacement => {
            for city in player_cities {
                let mut new_state = game_state.full_clone();
                let new_city = new_state.find_city(city.clone());
                let mut the_move = Move::new_place_army(app_state, new_state.clone(), new_city);
                new_state.score = 0;

                // Go deeper
                if new_state.depth != new_state.requested_depth {
                    let mut new_state = new_state.clone();
                    println!("Descending");

                    // Update player
                    let player_index = players.get(new_state.current_turn.unwrap()).unwrap().index;
                    new_state.players[player_index].armies_to_assign -= 1;

                    // Next player
                    if move_to_next_player(&mut new_state, app_state) {
                        // Turn is done
                        match &new_state.mode {
                            GameMode::ArmyPlacement => {
                                if new_state.players[player_index].armies_to_assign == 0 {
                                    println!("All armies placed");
                                    new_state.mode = GameMode::Game;
                                } else {
                                    println!("{} armies to place", new_state.players[player_index].armies_to_assign);
                                }
                            }
                            _ => panic!("Unknown mode"),
                        }
                    }

                    // Recurse (as next player)
                    new_state.depth += 1;
                    let additional_moves = &mut possible_moves(&new_state, app_state);
                    the_move.child_moves.append(additional_moves);
                }

                results.push(the_move);
            }
        }
        GameMode::Randomising => {}
        GameMode::Game => {}
    }

    println!("Returning: {}", results.len());
    results
}
