use crate::ai::game_state::GameState;
use crate::ai::scoring::score_for_city;
use crate::model::city::City;
use crate::model::city_state::CityState;
use crate::model::connection::Connection;
use rand::{thread_rng, Rng};
use skia_safe::Color;
use specs::prelude::*;
use specs_derive::Component;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AIProfile {
    pub search_depth: u8,
    pub city_size_multiplier: f32,
    pub army_multiplier: f32,
    pub army_same_territory: f32,
    pub army_bordering: f32,
    pub random_fraction: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {
    pub index: usize,
    pub name: String,
    pub score: i32,
    pub colours: Vec<Color>,
    pub cities: Vec<Arc<Mutex<CityState>>>,
    pub armies_to_assign: u32,
    pub profile: AIProfile,
}

impl Player {
    pub fn update_score(&self, world: &World, game_state: &GameState) {
        let entities = world.entities();
        let players = world.read_storage::<Player>();
        let cities = world.read_storage::<City>();
        let connections = world.read_storage::<Connection>();
        let player_entity = (&players, &entities).join().find(|(p, e)| std::ptr::eq(self, *p)).unwrap().1;

        // Per city
        let mut score = 0f32;
        for city in self.cities.iter() {
            let city_entity = cities.get(city.lock().unwrap().city);

            // Get connections
            let mut outgoing_connections = Vec::new();
            for connection_entity in &city_entity.unwrap().connections {
                let connection = connections.get(*connection_entity).unwrap();
                if connection.city1 == city.lock().unwrap().city {
                    let outgoing_city_state = game_state
                        .city_states
                        .iter()
                        .find(|city| city.lock().unwrap().city == connection.city2)
                        .unwrap();
                    outgoing_connections.push(outgoing_city_state.clone());
                }
            }

            // Update per city
            for city in &self.cities {
                score += score_for_city(
                    &cities,
                    city_entity.unwrap(),
                    city.lock().unwrap().clone(),
                    &outgoing_connections,
                    self,
                    player_entity,
                );
            }
        }

        /*        let players = app_state.world.read_storage::<Player>();
        let player_cities = game_state.get_player_cities();
        let player = players.get(game_state.current_player.unwrap()).unwrap(); */

        // Add randomness to keep it interesting and less easy to guess intentions
        let range = score * self.profile.random_fraction;
        let mut r = thread_rng();
        score += r.gen_range(0f32..range);

        //game_state.score = score as i32;
    }
    /*    fn run(&mut self, (mut components, cities): Self::SystemData) {
            for component in (&mut components).join() {
                component.score = 0;
                for city_state in &component.cities {
                    let city = cities.get(city_state.lock().unwrap().city);
                    /*component.score +=
                    score_for_city(&cities, city.unwrap(), city_state.lock().unwrap().clone(),
                    None, None, None);*/
                }
            }
        }
    }*/
}
