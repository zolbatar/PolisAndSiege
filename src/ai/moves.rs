use crate::app_state::AppState;
use crate::model::city::CityRR;
use rand::Rng;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use sdl2::sys::XFocusOutEvent;

#[derive(Debug, Default)]
pub enum MoveType {
    #[default]
    PlaceArmy,
    AttackCity,
}

#[derive(Default)]
pub struct Move {
    pub move_type: MoveType,
    pub city_source: Option<CityRR>,
    pub city_target: Option<CityRR>,
    pub child_moves: Vec<Move>,
    pub best_score: i32,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = self.city_source.as_ref().unwrap().borrow();
        //        let target = self.city_target.as_ref().unwrap().borrow();
        f.debug_struct("Move")
            .field("move_type", &self.move_type)
            .field("best_score", &self.best_score)
            .field("armies (source)", &source.armies)
            .field("+armies (source)", &source.additional_armies)
            .field("child_moves", &self.child_moves)
            .finish()
    }
}

impl Move {
    pub fn new_attack_city(city_source: &CityRR, city_target: &CityRR) -> Self {
        let new_city_source = Rc::new(RefCell::new(city_source.borrow().clone()));
        new_city_source.borrow_mut().original = Some(city_source.clone());
        let new_city_target = Rc::new(RefCell::new(city_target.borrow().clone()));
        new_city_target.borrow_mut().original = Some(city_target.clone());
        Self {
            move_type: MoveType::AttackCity,
            city_source: Some(new_city_source),
            city_target: Some(new_city_target),
            ..Default::default()
        }
    }

    pub fn new_place_army(city_source: &CityRR) -> Self {
        let new_city_source = Rc::new(RefCell::new(city_source.borrow().clone()));
        new_city_source.borrow_mut().original = Some(city_source.clone());
        new_city_source.borrow_mut().armies += 1;
        new_city_source.borrow_mut().additional_armies += 1;
        Self {
            move_type: MoveType::PlaceArmy,
            city_source: Some(new_city_source),
            ..Default::default()
        }
    }

    pub fn do_move(&self, app_state: &mut AppState) {
        let player = app_state.world_state.get_current_player();
        match self.move_type {
            MoveType::PlaceArmy => {
                self.city_source.as_ref().unwrap().borrow().original.clone().unwrap().borrow_mut().armies += 1;
                player.borrow_mut().armies_to_assign -= 1;
            }
            MoveType::AttackCity => {
                let mut rng = rand::thread_rng();
                let source = self.city_source.as_ref().unwrap().borrow().original.clone().unwrap();
                let target = self.city_target.as_ref().unwrap().borrow().original.clone().unwrap();
                let target_armies = target.borrow().armies;
                let mut source_armies = rng.gen_range(target_armies..(source.borrow().armies - 1));

                // Roll dice
                let mut dice_source = Vec::new();
                let mut dice_target = Vec::new();

                // source dice
                for _i in 0..source_armies {
                    let dice = rng.gen_range(1u8..=6u8);
                    dice_source.push(dice);
                }

                // Target dice
                for _i in 0..target_armies {
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
                    source.borrow().armies,
                    target.borrow().armies
                );
                
                for i in 0..dice_source.len() {
                    if i >= dice_target.len() || dice_source[i] > dice_target[i] {
                        if  target.borrow().armies == 0 {
                            break;
                        }
                        target.borrow_mut().armies -= 1;
                    } else {
                        source.borrow_mut().armies -= 1;
                        source_armies -= 1;
                    }
                }

                println!("After is {},{}.", source.borrow().armies, target.borrow().armies);

                // Take over!
                if target.borrow().armies == 0 {
                    // Take the city
                    target.borrow_mut().owner = source.borrow().owner.clone();
                    target.borrow_mut().armies = source_armies;
                    source.borrow_mut().armies -= source_armies;
                }
            }
        }
    }
}
