use specs::Entity;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct GameState {
    pub current_turn: Option<Entity>,
    pub actual_human: Option<Entity>,
    pub players: Vec<Entity>,
    pub territories: BTreeMap<String, Entity>,
}

impl GameState {}
