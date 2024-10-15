use crate::model::city_state::CityState;
use skia_safe::Color;
use specs::prelude::*;
use specs_derive::Component;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
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
