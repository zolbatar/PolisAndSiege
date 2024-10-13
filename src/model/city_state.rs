use specs::Entity;

#[derive(Debug, Clone, PartialEq)]
pub struct CityState {
    pub city: Entity,
    pub armies: u32,
    pub owner: Option<Entity>,
}
