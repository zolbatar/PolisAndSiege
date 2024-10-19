use crate::ai::moves::Move;
use crate::model::world_state::WorldState;

pub const MAXIMUM_ARMIES_PER_CITY: usize = 10;

pub fn ap_build_list_of_possibles(world_state: &WorldState, current_player: usize) -> Vec<Move> {
    let mut results = Vec::new();
    for (city_index, city) in world_state.cities.iter().enumerate() {
        if city.borrow().owner.unwrap() == current_player
            && world_state.cities[city_index].borrow().armies < MAXIMUM_ARMIES_PER_CITY
        {
            results.push(Move::new_place_army(city_index));
        }
    }
    results
}
