use crate::model::ai_profile::AIProfile;
use crate::model::city::CityAM;
use crate::model::player::PlayerAM;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct CityState {
    pub city: CityAM,
    pub armies: u32,
    pub additional_armies: u32,
    pub owner: Option<PlayerAM>,
    pub original: Option<CityStateAM>,
}

pub type CityStateAM = Arc<Mutex<CityState>>;

impl CityState {
    pub fn score(&self, aiprofile: &AIProfile) -> f32 {
        let mut score = 0f32;
        score += self.city.lock().unwrap().size as f32 * aiprofile.city_size_multiplier;
        score += self.armies as f32 * aiprofile.army_multiplier;

        // Logic for additional armies, extra score if bordering enemy concentrations
        for connection in self.city.lock().unwrap().connections.iter() {
            /*            let other_city_owner = connection.lock().unwrap().city2.lock().;

            // If enemy city, add a boost
            let boost = score;
            if other_city_owner.unwrap() != player_entity {
                score += city_state.additional_armies as f32 * player.profile.army_bordering;
                if other_city_entity.territory != city.territory {
                    score += city_state.additional_armies as f32 * player.profile.army_same_territory;
                }
            }
            let boost_diff = score - boost;
            if boost_diff > 0.0 {
                println!("City {} has a boost of {}", city.name, boost_diff);
            }*/
        }

        score
    }
}
