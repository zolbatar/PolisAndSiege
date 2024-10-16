use crate::app_state::AppState;
use crate::next_turn;
use std::fmt;
use crate::model::city::CityRR;

#[derive(Debug, Default)]
pub enum MoveType {
    #[default]
    PlaceArmy,
}

#[derive(Default)]
pub struct Move {
    pub move_type: MoveType,
    pub city_source: Option<CityRR>,
    pub _city_target: Option<CityRR>,
    pub child_moves: Vec<Move>,
    pub best_score: i32,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = self.city_source.as_ref().unwrap().borrow();
        f.debug_struct("Move")
            .field("move_type", &self.move_type)
            .field("best_score", &self.best_score)
            .field("armies", &source.armies)
            .field("+armies", &source.additional_armies)
            .field("child_moves", &self.child_moves)
            .finish()
    }
}

impl Move {
    pub fn new_place_army(city_source: CityRR) -> Self {
        city_source.borrow_mut().armies += 1;
        city_source.borrow_mut().additional_armies += 1;

        Self {
            move_type: MoveType::PlaceArmy,
            city_source: Some(city_source),
            ..Default::default()
        }
    }

    pub fn do_move_and_next_turn(&self, app_state: &mut AppState) {
        let player = app_state.world_state.get_current_player();
        match self.move_type {
            MoveType::PlaceArmy => {
                self.city_source.as_ref().unwrap().borrow().original.clone().unwrap().borrow_mut().armies += 1;
                player.borrow_mut().armies_to_assign -= 1;
            }
        }
        next_turn(app_state);
    }
}
