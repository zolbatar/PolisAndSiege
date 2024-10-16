use crate::app_state::AppState;
use crate::model::connection::ConnectionAM;
use crate::model::location::{calculate_distance, Location};
use crate::model::territory::TerritoryAM;
use petgraph::graph::NodeIndex;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct City {
    pub territory: TerritoryAM,
    pub connections: Vec<ConnectionAM>,
    pub location: Location,
    pub name: String,
    pub size: u8,
    pub node: NodeIndex,
    pub population: i64,
}

pub type CityAM = Arc<Mutex<City>>;

pub const SIZE: f32 = 3.0;
pub const SIZE_SELECTED: f32 = 4.0;
pub const MAXIMUM_LABEL_WIDTH: f32 = 32.0;

impl City {
    pub fn new(name: String, longitude: f32, latitude: f32, population: i64, territory: TerritoryAM) -> Self {
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
            territory,
            ..City::default()
        }
    }
}

// Function to select evenly spaced cities
pub fn select_evenly_spaced_cities(
    app_state: &mut AppState,
    num_cities_to_select: usize,
    territories: BTreeMap<String, TerritoryAM>,
) {
    for (territory_name, territory) in territories {
        let mut selected_cities = Vec::new();

        // Sort the cities by population (largest first)
        territory
            .lock()
            .unwrap()
            .cities
            .sort_by(|a, b| b.lock().unwrap().population.cmp(&a.lock().unwrap().population));

        // Loop through all cities
        for city in &territory.lock().unwrap().cities {
            let mut want = true;

            // Check distance to already selected cities
            for existing in &app_state.world_fixed.city_locations {
                if existing.p != city.lock().unwrap().location.p {
                    let dist = calculate_distance(&city.lock().unwrap().location, &existing);
                    if dist <= app_state.selection.minimum_allowed_distance {
                        want = false;
                        break;
                    }
                }
            }

            // If the city is far enough, select it
            if want {
                app_state.world_fixed.city_locations.push(city.lock().unwrap().location.clone());
                selected_cities.push(city.clone());

                // Stop if we have selected enough cities
                if selected_cities.len() >= num_cities_to_select {
                    break;
                }
            }
        }

        territory.lock().unwrap().cities = selected_cities;
        app_state.world_fixed.territories.insert(territory_name, territory);
    }
}
