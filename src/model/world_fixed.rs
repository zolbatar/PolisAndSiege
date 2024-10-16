use crate::model::city::CityAM;
use crate::model::city_state::CityStateAM;
use crate::model::connection::ConnectionAM;
use crate::model::location::Location;
use crate::model::territory::TerritoryAM;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct WorldFixed {
    pub territories: BTreeMap<String, TerritoryAM>,
    pub cities: Vec<CityAM>,
    pub city_locations: Vec<Location>,
    pub connections: Vec<ConnectionAM>,
    pub city_states_to_assign: Vec<CityStateAM>,
}
