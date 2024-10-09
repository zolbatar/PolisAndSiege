use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use crate::model::city::Owner;
use crate::model::connection::Connection;
use crate::model::territory::Territory;

#[derive(Clone)]
pub struct GameState {
    pub player: Owner,
    pub num_of_players: u8,
    pub territories: BTreeMap<String, Arc<Mutex<Territory>>>,
}

impl GameState {}
