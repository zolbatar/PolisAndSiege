#[derive(Debug, Clone)]
pub struct Profile {
    pub human: bool,
    pub search_depth: usize,
    pub city_size_multiplier: f32,
    pub army_multiplier: f32,
    pub army_same_territory: f32,
    pub army_bordering: f32,
    pub minimum_armies: usize,
    pub attack_delta_multiplier: f32,
}
