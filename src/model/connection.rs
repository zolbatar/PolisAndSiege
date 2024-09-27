use std::sync::Arc;
use crate::model::city::City;

pub struct Connection {
    city1: Arc<City>,
    city2: Arc<City>,
}

impl Connection {
    pub fn new(city1: Arc<City>, city2: Arc<City>) -> Connection {
        Connection {
            city1: Arc::clone(&city1),
            city2: Arc::clone(&city2),
        }
    }
}