use crate::model::ai_profile::AIProfile;
use crate::model::city_state::CityStateAM;
use rand::{thread_rng, Rng};
use skia_safe::Color;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Player {
    pub index: usize,
    pub name: String,
    pub score: i32,
    pub colours: Vec<Color>,
    pub cities: Vec<CityStateAM>,
    pub armies_to_assign: u32,
    pub profile: AIProfile,
}

pub type PlayerAM = Arc<Mutex<Player>>;

impl Player {
    pub fn is_human(&self) -> bool {
        self.profile.human
    }

    pub fn calculate_score(&mut self) {
        self.score = self.get_score() as i32
    }

    pub fn get_score(&mut self) -> f32 {
        let mut score = 0f32;
        for city_state in self.cities.iter() {
            let city = &city_state.as_ref().lock().unwrap().city;
            // Get connections
            let mut outgoing_connections = Vec::new();
            for connection in &city.lock().unwrap().connections {
                if Arc::ptr_eq(&connection.lock().unwrap().city1, city) {
                    outgoing_connections.push(connection.lock().unwrap().city2.clone());
                }
            }
        }

        // Update per city
        for city_state in self.cities.iter() {
            score += city_state.lock().unwrap().score(&self.profile);
        }
        /*            for city in &self.cities {
            score += score_for_city(
                &cities,
                city_entity.unwrap(),
                city.lock().unwrap().clone(),
                &outgoing_connections,
                self,
                player_entity,
            );
        }*/

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
