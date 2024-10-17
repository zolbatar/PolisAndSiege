use crate::ai::moves::Move;
use crate::model::player::PlayerRR;
use std::rc::Rc;

pub fn game_build_list_of_possibles(current_player: PlayerRR) -> Vec<Move> {
    let mut results = Vec::new();
    for city in &current_player.as_ref().borrow().cities {
        // Are there any enemy cities connected to attack?
        for connection in &city.borrow().connections {
            // Do we have enough armies?
            if city.borrow().armies >= current_player.borrow().profile.minimum_armies
                && connection.city1.borrow().armies
                    > connection.city2.borrow().armies + current_player.borrow().profile.minimum_army_delta
            {
                let owner = &connection.city2.borrow().owner.clone().unwrap();
                if !Rc::ptr_eq(owner, &current_player) {
                    results.push(Move::new_attack_city(city, &connection.city2));
                }
            }
        }
    }
    results
}
