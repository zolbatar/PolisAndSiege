use crate::model::city::CityRR;
use crate::model::connection::ConnectionArc;
use crate::model::location::Location;
use crate::model::territory::TerritoryArc;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct WorldFixed {
    pub territories: BTreeMap<String, TerritoryArc>,
    pub city_locations: Vec<Location>,
    pub connections: Vec<ConnectionArc>,
    pub cities_to_assign: Vec<CityRR>,
}
