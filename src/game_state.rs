use crate::model::city::Owner;
use crate::model::connection::Connection;
use crate::model::territory::Territory;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct GameState {
    pub player: Owner,
    pub num_of_players: u8,
    pub territories: BTreeMap<String, Arc<Mutex<Territory>>>,
    pub connections: Vec<Arc<Mutex<Connection>>>,
}

impl GameState {}
