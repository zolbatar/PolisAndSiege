use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use crate::model::city::Owner;
use crate::model::territory::Territory;

pub fn create_score(players: Vec<Owner>, territories: BTreeMap<String, Arc<Mutex<Territory>>>) -> HashMap<Owner, i32> {
    let mut m = HashMap::new();
    for owner in players {
        m.insert(owner, 0);
    }
    m
}