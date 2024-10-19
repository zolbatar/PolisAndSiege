use crate::model::profile::Profile;
use crate::model::world_fixed::WorldFixed;
use crate::model::world_state::WorldState;
use skia_safe::Color;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Player {
    pub index: usize,
    pub name: String,
    pub score: i32,
    pub colours: Vec<Color>,
    pub armies_to_assign: u32,
    pub armies_to_assign_fractional: f32,
    pub profile: Profile,
}

pub type PlayerRR = Rc<RefCell<Player>>;

impl Player {
    pub fn is_human(&self) -> bool {
        self.profile.human
    }

    pub fn calculate_score(&mut self, world_state: &WorldState, world_fixed: &WorldFixed) {
        self.score = self.get_score(world_state, world_fixed) as i32
    }

    pub fn get_score(&mut self, world_state: &WorldState, world_fixed: &WorldFixed) -> f32 {
        let mut score = 0f32;
        for (city_index, city) in world_state.cities.iter().enumerate() {
            let city = &world_state.cities[city_index];
            if city.borrow().owner.is_some() && city.borrow().owner.unwrap() == self.index {
                score += world_state.cities[city_index].borrow().score(world_state, world_fixed, &self.profile);
            }
        }
        score
    }
}
