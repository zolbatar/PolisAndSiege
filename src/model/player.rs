use crate::model::city::CCity;
use skia_safe::Color;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Default)]
pub enum PlayerType {
    Human,
    #[default]
    Computer,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct CPlayer {
    pub player_type: PlayerType,
    pub name: String,
    pub score: i32,
    pub colours: Vec<Color>,
    pub cities: Vec<Entity>,
}

pub struct SUpdateScores;
impl<'a> System<'a> for SUpdateScores {
    type SystemData = (WriteStorage<'a, CPlayer>, ReadStorage<'a, CCity>);

    fn run(&mut self, (mut components, cities): Self::SystemData) {
        for component in (&mut components).join() {
            component.score = 0;
            for city_entity in &component.cities {
                let city = cities.get(*city_entity);
                component.score += (city.unwrap().size as i32) * 10;
                component.score += city.unwrap().armies as i32;
            }
        }
    }
}
