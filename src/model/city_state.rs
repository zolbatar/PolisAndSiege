use specs::Entity;

#[derive(Debug, PartialEq, Clone)]
pub struct CityState {
    pub city: Entity,
    pub armies: u32,
    pub additional_armies: u32,
    pub owner: Option<Entity>,
}
