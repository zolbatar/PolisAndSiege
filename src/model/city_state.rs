use specs::Entity;

#[derive(Debug, PartialEq)]
pub struct CityState {
    pub city: Entity,
    pub armies: u32,
    pub owner: Option<Entity>,
}
