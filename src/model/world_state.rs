use std::cell::RefCell;
use std::rc::Rc;
use crate::app_state::GameMode;
use crate::model::city::CityRR;
use crate::model::player::PlayerRR;

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
            let mut new_city = city.borrow().clone();
            if city.borrow().original.is_some() {
                new_city.original = new_city.original.clone();
            } else {
                new_city.original = Some(city.clone());
            }
            cities.push(Rc::new(RefCell::new(new_city)));
        }
        Self {
            mode: self.mode.clone(),
            current_player: Some(players[self.current_player.as_ref().unwrap().borrow().index].clone()),
            players,
            cities,
        }
    }

    pub fn update_scores(&mut self) {
        for player in self.players.iter_mut() {
            player.borrow_mut().calculate_score();
        }
    }

    pub fn get_current_player(&self) -> PlayerRR {
        self.current_player.clone().unwrap()
    }
}
