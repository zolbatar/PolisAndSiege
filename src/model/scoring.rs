use crate::model::city::Owner;
use crate::model::territory::Territory;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

pub fn create_score(
    players: &Vec<Owner>,
    territories: &BTreeMap<String, Arc<Mutex<Territory>>>,
) -> HashMap<Owner, i32> {
    // First create an empty map to store the results
    let mut m = HashMap::new();
    for owner in players {
        m.insert(owner.clone(), 0);
    }

    // Now loop through all territories and cities
    for territory in territories.values() {
        for city in territory.lock().unwrap().cities.iter() {
            // Ownership is 50 points
            *m.get_mut(&city.lock().unwrap().owner).unwrap() += 50;
        }
    }

    // Print out
    for (player, score) in &m {
        println!("{:?} : {}", player, score)
    }

    m
}
