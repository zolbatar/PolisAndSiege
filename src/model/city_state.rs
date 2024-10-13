use specs::prelude::*;
use specs::Entity;
use specs_derive::Component;

#[derive(Component, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct CityState {
    pub city: Entity,
    pub armies: u32,
    pub owner: Option<Entity>,
}
