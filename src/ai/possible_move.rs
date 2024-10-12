use crate::app_state::{AppState, GameMode};
use crate::ai::game_state::GameState;
use specs::{Entity, WorldExt};
use crate::model::city::City;
use crate::model::territory::Territory;

#[derive(Debug)]
pub enum MoveType {
    PlaceArmy
}

#[derive(Debug)]
pub struct Move {
    move_type: MoveType,
    city: Entity,
}

impl Move {
    fn new_move(city: Entity) -> Self {
        Self {
            move_type: MoveType::PlaceArmy,
            city,
        }
    }
}

#[derive(Debug)]
pub struct Result {
    score: f32,
    moves: Vec<Move>,
    result: GameState,
}

pub fn possible_moves(game_state: &GameState, app_state: &AppState) -> Vec<Result> {
    let mut results = Vec::new();

    match game_state.mode {
        GameMode::ArmyPlacement => {
            let cities = app_state.world.read_storage::<City>();
            let territories = app_state.world.read_storage::<Territory>();
            for territory_entity in game_state.territories.values() {
                let territory = territories.get(*territory_entity).unwrap();
                for city_entity in &territory.cities {
                    let city = cities.get(*city_entity).unwrap();

                    // Is this the player's city?
                    if city.owner == game_state.current_turn {
                        results.push(Result {
                            score: 1.0,
                            moves: vec![Move::new_move(*city_entity)],
                            result: Default::default(),
                        })
                    }
                }
            }
        }
        _ => todo!("To do game mode {:?}", game_state.mode)
    }

    results
}