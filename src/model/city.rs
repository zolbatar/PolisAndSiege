use crate::app_state::AppState;
use crate::model::connection::ConnectionArc;
use crate::model::location::{calculate_distance, Location};
use crate::model::player::PlayerRR;
use crate::model::profile::Profile;
use crate::model::territory::{Territory, TerritoryArc};
use petgraph::graph::NodeIndex;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct City {
    pub territory: TerritoryArc,
    pub connections: Vec<ConnectionArc>,
    pub location: Location,
    pub name: String,
    pub size: u8,
    pub node: NodeIndex,
    pub population: i64,
    pub armies: usize,
    pub additional_armies: u32,
    pub attacking_delta: i32,
    pub owner: Option<PlayerRR>,
    pub original: Option<CityRR>,
}

pub type CityRR = Rc<RefCell<City>>;

pub const SIZE: f32 = 3.0;
pub const SIZE_SELECTED: f32 = 4.0;
pub const MAXIMUM_LABEL_WIDTH: f32 = 32.0;

impl City {
    pub fn new(name: String, longitude: f32, latitude: f32, population: i64) -> Self {
        let size = match population {
            0..150000 => 1,
            150000..500000 => 2,
            500000..2500000 => 3,
            2500000..5000000 => 4,
            _ => 5,
        };
        Self {
            name,
            size,
            location: Location::new(longitude, latitude),
            population,
            armies: 1,
            ..City::default()
        }
    }

    pub fn score(&self, profile: &Profile) -> f32 {
        let mut score = 0f32;
        score += self.size as f32 * profile.city_size_multiplier;
        score += self.armies as f32 * profile.army_multiplier;
        score += (self.attacking_delta + 10) as f32 * profile.attack_delta_multiplier;

        // Logic for additional armies, extra score if bordering enemy concentrations
        for connection in self.connections.iter() {
            let other_city_owner = &connection.city2.borrow().owner;
            let other_city_territory = &connection.city2.borrow().territory;

            // If enemy city, add a boost
            if other_city_owner.is_some() {
                let boost = score;
                if Rc::ptr_eq(&other_city_owner.as_ref().unwrap(), &self.owner.as_ref().unwrap()) {
                    score += self.additional_armies as f32 * profile.army_bordering;
                    if Arc::ptr_eq(other_city_territory, &self.territory) {
                        score += self.additional_armies as f32 * profile.army_same_territory;
                    }
                }
                let boost_diff = score - boost;
                if boost_diff > 0.0 {
                    println!("City {} has a boost of {}", self.name, boost_diff);
                }
            }
        }

        score
    }
}

// Function to select evenly spaced cities
pub fn select_evenly_spaced_cities(
    app_state: &mut AppState,
    num_cities_to_select: usize,
    territories: BTreeMap<String, Territory>,
) {
    for (territory_name, mut territory) in territories {
        let mut selected_cities = Vec::new();

        // Sort the cities by population (largest first)
        territory.cities.sort_by(|a, b| b.borrow().population.cmp(&a.borrow().population));

        // Loop through all cities
        for city in &territory.cities {
            let mut want = true;

            // Check distance to already selected cities
            for existing in &app_state.world_fixed.city_locations {
                if existing.p != city.borrow().location.p {
                    let dist = calculate_distance(&city.borrow().location, &existing);
                    if dist <= app_state.selection.minimum_allowed_distance {
                        want = false;
                        break;
                    }
                }
            }

            // If the city is far enough, select it
            if want {
                app_state.world_fixed.city_locations.push(city.borrow().location.clone());
                selected_cities.push(city.clone());

                // Stop if we have selected enough cities
                if selected_cities.len() >= num_cities_to_select {
                    break;
                }
            }
        }

        territory.cities = selected_cities;
        app_state.world_fixed.territories.insert(territory_name, territory.containerise());
    }
}
