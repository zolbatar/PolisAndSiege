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
