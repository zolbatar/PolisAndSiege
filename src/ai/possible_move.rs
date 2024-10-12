use crate::app_state::GameMode;
use crate::game_state::GameState;

pub enum MoveType {
    PlaceArmy
}

pub struct Move {
    move_type: MoveType,
}

pub struct Result {
    score: f32,
    moves: Vec<Move>,
    result: GameState,
}

pub fn possible_moves(game_state: &GameState) -> Vec<Result> {
    let mut results = Vec::new();

    match game_state.mode {
        GameMode::ArmyPlacement => {}
        _ => todo!("{:?}", game_state.mode)
    }

    results
}