use crate::app_state::AppState;
use crate::model::city_state::CityStateAM;
use crate::next_turn;
use std::fmt;

#[derive(Debug, Default)]
pub enum MoveType {
    #[default]
    PlaceArmy,
}

#[derive(Default)]
pub struct Move {
    pub move_type: MoveType,
    pub city_state_source: Option<CityStateAM>,
    pub _city_state_target: Option<CityStateAM>,
    pub child_moves: Vec<Move>,
    pub best_score: i32,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = &self.city_state_source.as_ref().unwrap().lock().unwrap();
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
    pub fn new_place_army(city_state_source: CityStateAM) -> Self {
        city_state_source.lock().unwrap().armies += 1;
        city_state_source.lock().unwrap().additional_armies += 1;

        Self {
            move_type: MoveType::PlaceArmy,
            city_state_source: Some(city_state_source),
            ..Default::default()
        }
    }

    pub fn do_move_and_next_turn(&mut self, app_state: &mut AppState) {
        let player = app_state.world_state.current_player.as_ref().unwrap();
        match self.move_type {
            MoveType::PlaceArmy => {
                let source = self.city_state_source.as_ref().unwrap().lock().unwrap();
                let original = source.original.clone().unwrap().clone();
                original.lock().unwrap().armies += 1;
                player.lock().unwrap().armies_to_assign -= 1;
            }
        }
        next_turn(app_state);
    }
}
