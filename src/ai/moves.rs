use crate::ai::army_placement::MAXIMUM_ARMIES_PER_CITY;
use crate::model::world_state::WorldState;
use rand::Rng;
use std::fmt;

#[derive(Debug, Default, PartialEq)]
pub enum MoveType {
    #[default]
    PlaceArmy,
    AttackCity,
}

#[derive(Default)]
pub struct Move {
    pub move_type: MoveType,
    pub city_source: Option<usize>,
    pub city_target: Option<usize>,
    pub child_moves: Vec<Move>,
    pub score_portion: i32,
    pub world_state: WorldState,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Move")
            .field("move_type", &self.move_type)
            .field("score_portion", &self.score_portion)
            .field("source", &self.city_source)
            .field("target", &self.city_target)
            .field("child_moves", &self.child_moves)
            .finish()
    }
}

impl Move {
    pub fn new_attack_city(city_source: usize, city_target: usize) -> Self {
        Self {
            move_type: MoveType::AttackCity,
            city_source: Some(city_source),
            city_target: Some(city_target),
            ..Default::default()
        }
    }

    pub fn new_place_army(city_source: usize) -> Self {
        Self {
            move_type: MoveType::PlaceArmy,
            city_source: Some(city_source),
            ..Default::default()
        }
    }

    pub fn do_move(&self, world_state: &mut WorldState) {
        let player = world_state.get_current_player();
        match self.move_type {
            MoveType::PlaceArmy => {
                let city = &world_state.cities[self.city_source.unwrap()];
                if city.borrow().armies < MAXIMUM_ARMIES_PER_CITY {
                    city.borrow_mut().armies += 1;
                    player.borrow_mut().armies_to_assign -= 1;
                }
            }
            MoveType::AttackCity => {
                let mut rng = rand::thread_rng();
                let source = self.city_source.unwrap();
                let target = self.city_target.unwrap();

                // Make sure we haven't already taken it and have enough armies
                if world_state.cities[source].borrow().owner != world_state.cities[target].borrow().owner
                    && world_state.cities[source].borrow().armies >= player.borrow().profile.minimum_armies
                {
                    let mut source_armies = world_state.cities[source].borrow().armies - 1;
                    let target_armies = world_state.cities[target].borrow().armies;

                    // Roll dice
                    let mut dice_source = Vec::new();
                    let mut dice_target = Vec::new();

                    // source dice
                    for _i in 1..=source_armies {
                        let dice = rng.gen_range(1u8..=6u8);
                        dice_source.push(dice);
                    }

                    // Target dice
                    for _i in 1..=target_armies {
                        let dice = rng.gen_range(1u8..=6u8);
                        dice_target.push(dice);
                    }

                    // Now order by
                    dice_source.sort();
                    dice_target.sort();

                    // And compare each
                    print!(
                        "Attacking with {}/{} (out of {},{}), ",
                        source_armies,
                        target_armies,
                        world_state.cities[source].borrow().armies,
                        world_state.cities[target].borrow().armies,
                    );

                    for i in 0..dice_source.len() {
                        if i >= dice_target.len() || dice_source[i] > dice_target[i] {
                            world_state.cities[target].borrow_mut().armies -= 1;
                            if world_state.cities[target].borrow().armies == 0 {
                                break;
                            }
                        } else {
                            world_state.cities[source].borrow_mut().armies -= 1;
                            source_armies -= 1;
                        }
                    }

                    println!(
                        "After is {},{}.",
                        world_state.cities[source].borrow().armies,
                        world_state.cities[target].borrow().armies
                    );

                    // Take over!
                    if world_state.cities[target].borrow().armies == 0 {
                        println!("City taken!");

                        // Take the city
                        let source_owner = world_state.cities[source].borrow().owner.unwrap();
                        world_state.cities[target].borrow_mut().owner = Some(source_owner);
                        world_state.cities[target].borrow_mut().armies = source_armies;
                    }
                }
            }
        }
    }
}
