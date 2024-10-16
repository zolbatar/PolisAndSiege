use crate::ai::moves::Move;
use crate::model::world_state::WorldState;
use std::sync::{Arc, Mutex};

pub fn ap_build_list_of_possibles(world_state: &WorldState) -> Vec<Move> {
    let mut results = Vec::new();
    let current_player = world_state.current_player.as_ref().unwrap();
    for city_state in &current_player.as_ref().lock().unwrap().cities {
        let new_city_state = Arc::new(Mutex::new(city_state.lock().unwrap().clone()));
        new_city_state.lock().unwrap().original = Some(city_state.clone());
        let the_move = Move::new_place_army(new_city_state);
        results.push(the_move);
    }
    results
}
