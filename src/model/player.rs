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
    pub cities: Vec<Entity>,
    pub armies_to_assign: u32,
}

fn score_for_city(score_in: i32, city: &City, city_state: &CityState) -> i32 {
    let mut score = score_in;
    score += (city.size as i32) * 10;
    score += city_state.armies as i32;
    score
}

pub struct SUpdateScores;
impl<'a> System<'a> for SUpdateScores {
    type SystemData = (WriteStorage<'a, Player>, ReadStorage<'a, City>, ReadStorage<'a, CityState>);

    fn run(&mut self, (mut components, cities, city_states): Self::SystemData) {
        for component in (&mut components).join() {
            component.score = 0;
            for city_entity in &component.cities {
                let city_state = city_states.get(*city_entity);
                let city = cities.get(city_state.unwrap().city);
                let city_state = city_states.get(*city_entity);
                component.score = score_for_city(component.score, city.unwrap(), city_state.unwrap());
            }
        }
    }
}

