use crate::ai::moves::Move;
use crate::model::player::PlayerRR;
use rand::distributions::uniform::SampleBorrow;
use rand::Rng;
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

pub fn do_attack_city(player: PlayerRR, the_move: &Move) {
    let mut rng = rand::thread_rng();
    let source = the_move.city_source.as_ref().unwrap().borrow();
    let target = the_move.city_target.as_ref().unwrap().borrow();
    let source_armies = source.armies;
    let diff = target.armies - source_armies;
    let target_armies = rng.gen_range(player.borrow().profile.minimum_army_delta..=diff);

    // Roll dice
    let mut dice_source = Vec::new();
    let mut dice_target = Vec::new();

    // source dice
    for i in 0..source_armies {
        let dice = rng.gen_range(1u8..=6u8);
        dice_source.push(dice);
    }

    // Target dice
    for i in 0..target_armies {
        let dice = rng.gen_range(1u8..=6u8);
        dice_target.push(dice);
    }

    // Now order by
    dice_source.sort();
    dice_target.sort();

    // And compare each
    dice_source.truncate(dice_target.len());
    let mut source_win = 0usize;
    let mut target_win = 0usize;
    for i in 0..dice_source.len() {
        if dice_source[i] > dice_target[i] {
            source_win += 1;
        } else {
            target_win += 1;
        }
    }

    // Now do the result
    if source_win >= target.armies {}
}
