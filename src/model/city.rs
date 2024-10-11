use crate::app_state::AppState;
use crate::model::location::{calculate_distance, Location};
use petgraph::graph::NodeIndex;
use specs::prelude::*;
use specs::Entity;
use specs_derive::Component;
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Component, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct City {
    pub territory: Entity,
    pub location: Location,
    pub name: String,
    pub size: u8,
    pub armies: u8,
    pub owner: Option<Entity>,
    pub node: NodeIndex,
}

pub struct CityTemporary {
    pub name: String,
    pub location: Location,
    pub population: i64,
}

pub const SIZE: f32 = 3.0;
pub const SIZE_SELECTED: f32 = 4.0;
pub const MAXIMUM_LABEL_WIDTH: f32 = 32.0;

impl CityTemporary {
    pub fn new(name: String, longitude: f32, latitude: f32, population: i64) -> Self {
        CityTemporary {
            name,
            location: Location::new(longitude, latitude),
            population,
        }
    }
}

// Function to select evenly spaced cities
pub fn select_evenly_spaced_cities(
    app_state: &mut AppState,
    territory_city: &mut HashMap<String, Vec<CityTemporary>>,
    num_cities_to_select: usize,
) -> HashMap<String, Vec<CityTemporary>> {
    let mut out: HashMap<String, Vec<CityTemporary>> = HashMap::new();
    for (territory, cities) in territory_city {
        let mut selected_cities: Vec<CityTemporary> = Vec::new();

        // Sort the cities by population (largest first)
        cities.sort_by(|a, b| b.population.cmp(&a.population)); // Sort largest first

        // Loop through all cities
        let mut i = 0;
        while i < cities.len() {
            let city = &cities[i];
            let mut want = true;

            // Check distance to already selected cities
            for existing in app_state.items.existing_cities.iter() {
                if existing.p != city.location.p {
                    let dist = calculate_distance(&city.location, &existing);
                    if dist <= app_state.selection.minimum_allowed_distance {
                        want = false;
                        break;
                    }
                }
            }

            // If the city is far enough, select it
            if want {
                app_state.items.existing_cities.push(city.location.clone());
                selected_cities.push(cities.remove(i));

                // Stop if we have selected enough cities
                if selected_cities.len() >= num_cities_to_select {
                    break;
                }
            }
            i += 1;
        }

        out.insert(territory.clone(), selected_cities);
    }
    out
}
