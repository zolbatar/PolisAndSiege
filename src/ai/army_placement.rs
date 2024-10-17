use crate::ai::moves::Move;
use crate::model::player::PlayerRR;

pub fn ap_build_list_of_possibles(current_player: PlayerRR) -> Vec<Move> {
    let mut results = Vec::new();
    for city in &current_player.as_ref().borrow().cities {
        results.push(Move::new_place_army(city));
    }
    results
}
