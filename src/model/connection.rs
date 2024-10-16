use crate::model::city::CityAM;
use crate::model::location::calculate_distance;
use crate::model::territory::TerritoryAM;
use crate::model::world_fixed::WorldFixed;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::prelude::UnGraph;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub const LINE_WIDTH: f32 = 0.25;

#[derive(Debug, Clone)]
pub struct Connection {
    pub city1: CityAM,
    pub city2: CityAM,
    pub render: bool,
    pub same_territory: bool,
}

pub type ConnectionAM = Arc<Mutex<Connection>>;

fn build_territory_connections(
    connections: &mut Vec<ConnectionAM>,
    territory1: &TerritoryAM,
    territory2: &TerritoryAM,
    num_connections: usize,
) {
    let mut m1 = BTreeMap::new();
    let mut m2 = BTreeMap::new();

    for city1 in territory1.lock().unwrap().cities.iter() {
        for city2 in territory2.lock().unwrap().cities.iter() {
            if !Arc::ptr_eq(city1, city2) {
                let distance = calculate_distance(&city1.lock().unwrap().location, &city2.lock().unwrap().location);
                m1.insert(
                    distance as usize,
                    Arc::new(Mutex::new(Connection {
                        city1: city1.clone(),
                        city2: city2.clone(),
                        render: true,
                        same_territory: false,
                    })),
                );
                m2.insert(
                    distance as usize,
                    Arc::new(Mutex::new(Connection {
                        city2: city1.clone(),
                        city1: city2.clone(),
                        render: false,
                        same_territory: false,
                    })),
                );
            }
        }
    }
    assert!(m1.len() >= num_connections);
    assert!(m2.len() >= num_connections);

    // Shrink and return
    let mut iter = m1.iter();
    for _ in 0..num_connections {
        let v = iter.next().unwrap();
        connections.push(v.1.clone());
    }
    let mut iter = m2.iter();
    for _ in 0..num_connections {
        let v = iter.next().unwrap();
        connections.push(v.1.clone());
    }
}

pub fn build_connections(world_fixed: &mut WorldFixed) {
    let mut connections = Vec::new();
    for territory in world_fixed.territories.values_mut() {
        let mut graph = UnGraph::new_undirected();
        let cities = &territory.lock().unwrap().cities;

        // Cities
        for city in cities {
            let node = graph.add_node(city.clone());
            city.lock().unwrap().node = node;
        }

        // Distances
        for city1 in cities {
            for city2 in cities {
                if !Arc::ptr_eq(city1, city2) {
                    let city1un = &city1.lock().unwrap();
                    let city2un = &city2.lock().unwrap();
                    let distance = calculate_distance(&city1un.location, &city2un.location);
                    graph.add_edge(city1un.node, city2un.node, distance);
                }
            }
        }

        // Get connections
        let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&graph));
        for edge in mst.raw_edges() {
            let _weight = edge.weight;
            let source = graph[edge.source()].clone();
            let target = graph[edge.target()].clone();

            // Create connections, but only render one
            let connection1 = Arc::new(Mutex::new(Connection {
                city1: source.clone(),
                city2: target.clone(),
                render: true,
                same_territory: true,
            }));
            let connection2 = Arc::new(Mutex::new(Connection {
                city1: target.clone(),
                city2: source.clone(),
                render: true,
                same_territory: true,
            }));
            connections.push(connection1.clone());
            connections.push(connection2.clone());
            source.lock().unwrap().connections.push(connection1);
            target.lock().unwrap().connections.push(connection2);
        }
    }
    println!("Built intra-territory connections");

    let na = world_fixed.territories.get("North America").unwrap();
    let la = world_fixed.territories.get("Latin America and the Caribbean").unwrap();
    let eu = world_fixed.territories.get("Europe").unwrap();
    let me = world_fixed.territories.get("Middle East and North Africa").unwrap();
    let ee = world_fixed.territories.get("Eastern Europe").unwrap();
    let af = world_fixed.territories.get("Sub-Saharan Africa").unwrap();
    let asia = world_fixed.territories.get("Asia").unwrap();
    let au = world_fixed.territories.get("Australia and New Zealand").unwrap();

    // Now we need inter-territory connections
    build_territory_connections(&mut connections, na, la, 2);
    build_territory_connections(&mut connections, na, eu, 1);
    build_territory_connections(&mut connections, eu, ee, 2);
    build_territory_connections(&mut connections, eu, me, 2);
    build_territory_connections(&mut connections, af, me, 2);
    build_territory_connections(&mut connections, af, la, 1);
    build_territory_connections(&mut connections, asia, me, 2);
    build_territory_connections(&mut connections, asia, au, 1);
    build_territory_connections(&mut connections, asia, ee, 2);
    println!("Built inter-territory connections");

    world_fixed.connections = connections;
}
