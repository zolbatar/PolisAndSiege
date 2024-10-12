use crate::app_state::{AppState, GameMode};
use crate::ai::game_state::GameState;
use specs::{Entity, WorldExt};
use crate::model::city::City;
use crate::model::player::Player;
use crate::model::territory::Territory;
use crate::next_turn;

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
    fn new_place_army(city: Entity) -> Self {
        Self {
            move_type: MoveType::PlaceArmy,
            city,
        }
    }
}

#[derive(Debug)]
pub struct Result {
    pub(crate) score: i32,
    the_move: Move,
    result: GameState,
}

impl Result {
    pub fn do_move(&self, app_state: &mut AppState) {
        {
            let mut players = app_state.world.write_storage::<Player>();
            let current_player = players.get_mut(app_state.current_player).unwrap();
            let mut cities = app_state.world.write_storage::<City>();
            match self.the_move.move_type {
                MoveType::PlaceArmy => {
                    cities.get_mut(self.the_move.city).unwrap().armies += 1;
                    current_player.armies_to_assign -= 1;
                    /*println!("Assign: {} {} {}", current_player.index, cities.get(self.the_move
                        .city).unwrap().name, cities.get(self.the_move
                        .city).unwrap().armies);*/
                }
            }
        }
        next_turn(app_state);
    }
}

pub fn possible_moves(game_state: &GameState, app_state: &AppState) -> Vec<Result> {
    let mut results = Vec::new();
    let cities = app_state.world.read_storage::<City>();
    let territories = app_state.world.read_storage::<Territory>();
    let players = app_state.world.read_storage::<Player>();
    let current_player = players.get(game_state.current_turn.unwrap()).unwrap();

    match game_state.mode {
        GameMode::ArmyPlacement => {
            for territory_entity in game_state.territories.values() {
                let territory = territories.get(*territory_entity).unwrap();
                for city_entity in &territory.cities {
                    let city = cities.get(*city_entity).unwrap();

                    // Is this the player's city?
                    if city.owner == game_state.current_turn {
                        results.push(Result {
                            score: current_player.score + 1,
                            the_move: Move::new_place_army(*city_entity),
                            result: Default::default(),
                        })
                    }
                }
            }
        }
        GameMode::Game => {}
        _ => todo!("To do game mode {:?}", game_state.mode)
    }

    results
}
