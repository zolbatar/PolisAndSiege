use crate::app_state::GameMode;
use crate::model::city::{City, CityRR};
use crate::model::player::PlayerRR;
use crate::model::world_fixed::WorldFixed;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct WorldState {
    pub mode: GameMode,
    pub current_player: Option<PlayerRR>,
    pub players: Vec<PlayerRR>,
    pub cities: Vec<CityRR>,
}

impl WorldState {
    pub fn deep_clone(&self) -> Self {
        let mut players = Vec::new();
        for player in &self.players {
            players.push(Rc::new(RefCell::new(player.borrow().clone())));
        }
        let mut cities = Vec::new();
        for city in &self.cities {
            let new_city = City::full_clone(city);
            cities.push(new_city);
        }
        Self {
            mode: self.mode.clone(),
            current_player: Some(players[self.current_player.as_ref().unwrap().borrow().index].clone()),
            players,
            cities,
        }
    }

    pub fn update_scores(&mut self, world_fixed: &WorldFixed) {
        for player in self.players.iter() {
            player.borrow_mut().calculate_score(self, world_fixed);
        }
    }

    pub fn get_current_player(&self) -> PlayerRR {
        self.current_player.clone().unwrap()
    }

    pub fn get_current_player_index(&self) -> usize {
        assert!(self.current_player.as_ref().unwrap().borrow().index < self.players.len());
        self.current_player.as_ref().unwrap().borrow().index
    }

    pub fn get_player_for_index(&self, player: usize) -> PlayerRR {
        assert!(player < self.players.len());
        self.players[player].clone()
    }
}
