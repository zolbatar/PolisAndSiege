use crate::app_state::GameMode;
use specs::Entity;
use std::collections::BTreeMap;

#[derive(Default, Debug, Clone)]
pub struct GameState {
    pub current_turn: Option<Entity>,
    pub actual_human: Option<Entity>,
    pub players: Vec<Entity>,
    pub territories: BTreeMap<String, Entity>,
    pub mode: GameMode,
    pub depth: u8,
    pub requested_depth: u8,
}

impl GameState {}
