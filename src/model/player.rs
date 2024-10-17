use crate::model::city::CityRR;
use crate::model::profile::Profile;
use rand::{thread_rng, Rng};
use skia_safe::Color;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Player {
    pub index: usize,
    pub name: String,
    pub score: i32,
    pub colours: Vec<Color>,
    pub cities: Vec<CityRR>,
    pub armies_to_assign: u32,
    pub armies_to_assign_fractional: f32,
    pub profile: Profile,
}

pub type PlayerRR = Rc<RefCell<Player>>;

impl Player {
    pub fn is_human(&self) -> bool {
        self.profile.human
    }

    pub fn calculate_score(&mut self) {
        self.score = self.get_score() as i32
    }

    pub fn get_score(&mut self) -> f32 {
        let mut score = 0f32;
        for city in self.cities.iter() {
            score += city.borrow().score(&self.profile);
        }

        // Add randomness to keep it interesting and less easy to guess intentions
        let mut range = score * self.profile.random_fraction;
        if range < 5.0 {
            range = 5.0
        }
        let mut r = thread_rng();
        score += r.gen_range(0f32..range);
        score
    }
}
