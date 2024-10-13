use std::sync::{Arc, Mutex};
use crate::model::city::City;
use skia_safe::Color;
use specs::prelude::*;
use specs_derive::Component;
use crate::model::city_state::CityState;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Player {
    pub index: usize,
    pub name: String,
    pub score: i32,
    pub colours: Vec<Color>,
    pub cities: Vec<Arc<Mutex<CityState>>>,
    pub armies_to_assign: u32,
}

fn score_for_city(score_in: i32, city: &City, city_state: Arc<Mutex<CityState>>) -> i32 {
    let mut score = score_in;
    score += (city.size as i32) * 10;
    score += city_state.lock().unwrap().armies as i32;
    score
}

pub struct SUpdateScores;
impl<'a> System<'a> for SUpdateScores {
    type SystemData = (WriteStorage<'a, Player>, ReadStorage<'a, City>);

    fn run(&mut self, (mut components, cities): Self::SystemData) {
        for component in (&mut components).join() {
            component.score = 0;
            for city_state in &component.cities {
                let city = cities.get(city_state.lock().unwrap().city);
                component.score = score_for_city(component.score, city.unwrap(), city_state.clone());
            }
        }
    }
}

