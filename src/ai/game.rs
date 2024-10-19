use crate::ai::moves::Move;
use crate::model::world_state::WorldState;
use rand::distributions::uniform::SampleBorrow;

pub fn game_build_list_of_possibles(
    world_state: &WorldState,
    current_player: usize,
) -> Vec<Move> {
    let mut results: Vec<Move> = Vec::new();
    let player = world_state.get_player_for_index(current_player);
    for city in world_state.cities.iter() {
        if city.borrow().owner.unwrap() == current_player {
            // Are there any enemy cities connected to attack?
            for connection in city.borrow().statics.borrow().connections.iter() {
                let city1 = &world_state.cities[connection.city1];
                let other_city = &world_state.cities[connection.city2];

                // Do we already have this?
                let others = results.iter().filter(|result| result.city_target.unwrap() == connection.city2).count();
                if others > 0 {
                    break;
                }

                assert_eq!(city1.borrow().owner.unwrap(), current_player);
                let owner = other_city.borrow().owner.unwrap();
                if owner != current_player {
                    // Do we have enough armies?
                    if city.borrow().armies >= player.borrow().profile.minimum_armies {
                        results.push(Move::new_attack_city(connection.city1, connection.city2));
                    }
                }
            }
        }
    }
    results
}
