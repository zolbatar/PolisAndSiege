use crate::app_state::AppState;
use crate::model::connection::ConnectionArc;
use crate::model::location::{calculate_distance, Location};
use crate::model::profile::Profile;
use crate::model::territory::Territory;
use crate::model::world_fixed::WorldFixed;
use crate::model::world_state::WorldState;
use petgraph::graph::NodeIndex;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct CityStatic {
    pub territory_name: String,
    pub connections: Vec<ConnectionArc>,
    pub name: String,
    pub node: NodeIndex,
    pub population: i64,
    pub location: Location,
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct City {
    pub statics: CityStaticRR,
    pub size: u8,
    pub armies: usize,
    pub owner: Option<usize>,
}

pub type CityStaticRR = Rc<RefCell<CityStatic>>;
pub type CityRR = Rc<RefCell<City>>;

pub const SIZE: f32 = 3.0;
pub const SIZE_SELECTED: f32 = 4.0;
pub const MAXIMUM_LABEL_WIDTH: f32 = 32.0;

impl City {
    pub fn new(
        name: String,
        longitude: f32,
        latitude: f32,
        population: i64,
        territory_name: String,
    ) -> Self {
        let size = match population {
            0..150000 => 1,
            150000..500000 => 2,
            500000..2500000 => 3,
            2500000..5000000 => 4,
            _ => 5,
        };
        let statics = Rc::new(RefCell::new(CityStatic {
            territory_name,
            name,
            location: Location::new(longitude, latitude),
            population,
            ..CityStatic::default()
        }));

        Self {
            statics,
            size,
            armies: 1,
            owner: None,
        }
    }

    pub fn full_clone(city_rr: &CityRR) -> CityRR {
        let cloned_raw = city_rr.borrow().clone();
        Rc::new(RefCell::new(cloned_raw))
    }

    pub fn score(&self, world_state: &WorldState, world_fixed: &WorldFixed, profile: &Profile) -> f32 {
        let mut score = 0f32;
        score += self.size as f32 * profile.city_size_multiplier;
        score += self.armies as f32 * profile.army_multiplier;

        // Logic for additional armies, extra score if bordering enemy concentrations
        for connection in self.statics.borrow().connections.iter() {
            let other_city = &world_state.cities[connection.city2];
            let other_city_owner = other_city.borrow().owner;
            let other_city_territory = other_city.borrow().statics.borrow().territory_name.clone();
            score += self.armies as f32 * profile.army_multiplier;

            // If enemy city, add a boost
            if other_city_owner.is_some() {
                let boost = score;
                if other_city_owner != self.owner {
                    score += self.armies as f32 * profile.army_bordering;
                    if other_city_territory.eq(&self.statics.borrow().territory_name) {
                        score += self.armies as f32 * profile.army_same_territory;
                    }
                }
                let boost_diff = score - boost;
                /*                if boost_diff > 0.0 {
                    println!("City {} has a boost of {}", self.name, boost_diff);
                }*/
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
    let world_state = &mut app_state.world_state;
    let world_fixed = &mut app_state.world_fixed;
    let mut city_index = 0usize;
    for (territory_name, mut territory) in territories {
        let mut selected_cities = Vec::new();

        // Sort the cities by population (largest first)
        territory.cities.sort_by(|a, b| {
            b.borrow().statics.borrow().population.cmp(&a.borrow().statics.borrow().population)
        });

        // Loop through all cities
        for city in &territory.cities {
            let mut want = true;

            // Check distance to already selected cities
            for existing in world_fixed.city_locations.iter() {
                if existing.p != city.borrow().statics.borrow().location.p {
                    let dist = calculate_distance(
                        &city.borrow().statics.borrow().location,
                        existing,
                    );
                    if dist <= app_state.selection.minimum_allowed_distance {
                        want = false;
                        break;
                    }
                }
            }

            // If the city is far enough, select it
            if want {
                city.borrow().statics.borrow_mut().index = city_index;
                city_index += 1;
                world_fixed
                    .city_locations
                    .push(city.borrow().statics.borrow().location.clone());
                selected_cities.push(city.clone());

                // Stop if we have selected enough cities
                if selected_cities.len() >= num_cities_to_select {
                    break;
                }
            }
        }

        territory.cities = selected_cities;
        for city in &territory.cities {
            world_state.cities.push(city.clone());
        }
        let contained = territory.containerise(world_fixed);
        world_fixed.territories.insert(territory_name, contained);
    }
}
