use crate::model::city::City;
use skia_safe::Color;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Player {
    pub index: usize,
    pub name: String,
    pub score: i32,
    pub colours: Vec<Color>,
    pub cities: Vec<Entity>,
    pub armies_to_assign: i32,
}

pub struct SUpdateScores;
impl<'a> System<'a> for SUpdateScores {
    type SystemData = (WriteStorage<'a, Player>, ReadStorage<'a, City>);

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
