use crate::model::city::City;
use crate::model::city_state::CityState;
use skia_safe::Color;
use specs::prelude::*;
use specs_derive::Component;
use std::sync::{Arc, Mutex};

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

pub fn score_for_city(city: &City, city_state: &CityState) -> i32 {
    let mut score = 0;
    score += (city.size as i32) * 10;
    score += city_state.armies as i32;

    // Logic for additional armies, extra score if bordering enemy concentrations

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
                component.score += score_for_city(city.unwrap(), &city_state.lock().unwrap());
            }
        }
    }
}
