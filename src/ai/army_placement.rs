use std::cell::RefCell;
use std::rc::Rc;
use crate::ai::moves::Move;
use crate::model::world_state::WorldState;

pub fn ap_build_list_of_possibles(world_state: &WorldState) -> Vec<Move> {
    let mut results = Vec::new();
    let current_player = world_state.get_current_player();
    for city in &current_player.as_ref().borrow().cities {
        let new_city = Rc::new(RefCell::new(city.borrow().clone()));
        new_city.borrow_mut().original = Some(city.clone());
        let the_move = Move::new_place_army(new_city);
        results.push(the_move);
    }
    results
}
