use crate::model::location::calculate_distance;
use crate::model::territory::TerritoryArc;
use crate::model::world_fixed::WorldFixed;
use crate::model::world_state::WorldState;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::prelude::UnGraph;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;

pub const LINE_WIDTH: f32 = 0.25;

#[derive(Debug, Clone)]
pub struct Connection {
    pub city1: usize,
    pub city2: usize,
    pub render: bool,
    pub same_territory: bool,
}

pub type ConnectionArc = Arc<Connection>;

fn build_territory_connections(
    connections: &mut Vec<ConnectionArc>,
    territory1: &TerritoryArc,
    territory2: &TerritoryArc,
    num_connections: usize,
) {
    let mut m1 = BTreeMap::new();
    let mut m2 = BTreeMap::new();

    for city1 in territory1.cities.iter() {
        for city2 in territory2.cities.iter() {
            if !Rc::ptr_eq(city1, city2) {
                let distance = calculate_distance(
                    &city1.borrow().statics.borrow().location,
                    &city2.borrow().statics.borrow().location,
                );
                m1.insert(
                    distance as usize,
                    Arc::new(Connection {
                        city1: city1.borrow().statics.borrow().index,
                        city2: city2.borrow().statics.borrow().index,
                        render: true,
                        same_territory: false,
                    }),
                );
                m2.insert(
                    distance as usize,
                    Arc::new(Connection {
                        city2: city1.borrow().statics.borrow().index,
                        city1: city2.borrow().statics.borrow().index,
                        render: false,
                        same_territory: false,
                    }),
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

pub fn build_connections(world_state: &WorldState, world_fixed: &mut WorldFixed) {
    let mut connections = Vec::new();
    let mut connections_in = world_fixed.territories.clone();
    for territory in connections_in.values() {
        let mut graph = UnGraph::new_undirected();
        let cities = &territory.cities;

        // Cities
        for (city_index, city) in cities.iter().enumerate() {
            let node = graph.add_node(city);
            city.borrow().statics.borrow_mut().node = node;
        }

        // Distances
        for city1 in cities.iter() {
            for city2 in cities.iter() {
                if !Rc::ptr_eq(city1, city2) {
                    let distance = calculate_distance(
                        &city1.borrow().statics.borrow().location,
                        &city2.borrow().statics.borrow().location,
                    );
                    graph.add_edge(
                        city1.borrow().statics.borrow().node,
                        city2.borrow().statics.borrow().node,
                        distance,
                    );
                }
            }
        }

        // Get connections
        let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&graph));
        for edge in mst.raw_edges() {
            let _weight = edge.weight;
            let source = graph[edge.source()];
            let target = graph[edge.target()];

            // Create connections, but only render one
            let connection1 = Arc::new(Connection {
                city1: source.borrow().statics.borrow().index,
                city2: target.borrow().statics.borrow().index,
                render: true,
                same_territory: true,
            });
            let connection2 = Arc::new(Connection {
                city1: target.borrow().statics.borrow().index,
                city2: source.borrow().statics.borrow().index,
                render: false,
                same_territory: true,
            });
            connections.push(connection1.clone());
            connections.push(connection2.clone());
            source.borrow().statics.borrow_mut().connections.push(connection1);
            target.borrow().statics.borrow_mut().connections.push(connection2);
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
