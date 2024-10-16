use crate::app_state::GameMode;
use crate::model::city_state::CityStateAM;
use crate::model::player::PlayerAM;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct WorldState {
    pub mode: GameMode,
    pub current_player: Option<PlayerAM>,
    pub players: Vec<PlayerAM>,
    pub city_states: Vec<CityStateAM>,
}

impl WorldState {
    pub fn deep_clone(&self) -> Self {
        let mut players = Vec::new();
        for player in &self.players {
            players.push(Arc::new(Mutex::new(player.lock().unwrap().clone())));
        }
        let mut city_states = Vec::new();
        for city_state in &self.city_states {
            let mut new_city_state = city_state.lock().unwrap().clone();
            if city_state.lock().unwrap().original.is_some() {
                new_city_state.original = new_city_state.original.clone();
            } else {
                new_city_state.original = Some(city_state.clone());
            }
            city_states.push(Arc::new(Mutex::new(new_city_state)));
        }
        Self {
            mode: self.mode.clone(),
            current_player: Some(players[self.current_player.as_ref().unwrap().lock().unwrap().index].clone()),
            players,
            city_states,
        }
    }
    pub fn update_scores(&mut self) {
        for player in self.players.iter_mut() {
            player.lock().unwrap().calculate_score();
        }
    }
}
