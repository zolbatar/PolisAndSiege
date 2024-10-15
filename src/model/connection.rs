use crate::app_state::AppState;
use crate::model::city::City;
use crate::model::location::calculate_distance;
use crate::model::territory::Territory;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::prelude::UnGraph;
use specs::prelude::*;
use specs::{Join, WorldExt};
use specs_derive::Component;
use std::collections::BTreeMap;

pub const LINE_WIDTH: f32 = 0.25;

#[derive(Component, Debug, PartialEq, Clone)]
#[storage(VecStorage)]
pub struct Connection {
    pub city1: Entity,
    pub city2: Entity,
    pub render: bool,
    pub same_territory: bool,
}

fn build_territory_connections(
    app_state: &mut AppState,
    connections: &mut Vec<Connection>,
    territory1_entity: Entity,
    territory2_entity: Entity,
    num_connections: usize,
) {
    let mut m1 = BTreeMap::new();
    let mut m2 = BTreeMap::new();
    let cities = app_state.world.read_storage::<City>();
    let territories = app_state.world.read_storage::<Territory>();

    let territory1 = territories.get(territory1_entity).unwrap();
    let territory2 = territories.get(territory2_entity).unwrap();
    for city1_state in territory1.cities.iter() {
        for city2_state in territory2.cities.iter() {
            let city1 = cities.get(city1_state.lock().unwrap().city).unwrap();
            let city2 = cities.get(city2_state.lock().unwrap().city).unwrap();
            if city1 != city2 {
                let distance = calculate_distance(&city1.location, &city2.location);
                m1.insert(
                    distance as usize,
                    Connection {
                        city1: city1_state.lock().unwrap().city,
                        city2: city2_state.lock().unwrap().city,
                        render: true,
                        same_territory: false,
                    },
                );
                m2.insert(
                    distance as usize,
                    Connection {
                        city2: city1_state.lock().unwrap().city,
                        city1: city2_state.lock().unwrap().city,
                        render: false,
                        same_territory: false,
                    },
                );
            }
        }
    }

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

pub fn build_connections(app_state: &mut AppState) {
    let mut connections = Vec::new();
    for territory in app_state.world.read_storage::<Territory>().join() {
        let mut graph = UnGraph::new_undirected();

        // Cities
        let mut cities = app_state.world.write_storage::<City>();
        for city_state in &territory.cities {
            let city = cities.get_mut(city_state.lock().unwrap().city).unwrap();
            let node = graph.add_node(city_state.lock().unwrap().city);
            city.node = node;
        }

        // Distances
        for city1_state in &territory.cities {
            for city2_state in &territory.cities {
                let city1 = cities.get(city1_state.lock().unwrap().city).unwrap();
                let city2 = cities.get(city2_state.lock().unwrap().city).unwrap();
                if city1 != city2 {
                    let distance = calculate_distance(&city1.location, &city2.location);
                    graph.add_edge(city1.node, city2.node, distance);
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
            connections.push(Connection {
                city1: source,
                city2: target,
                render: true,
                same_territory: true,
            });
            connections.push(Connection {
                city2: source,
                city1: target,
                render: false,
                same_territory: true,
            });
        }
    }

    // Now we need inter-territory connections
    {
        //let territories = app_state.world.read_storage::<_Territory>();
        build_territory_connections(
            app_state,
            &mut connections,
            app_state.items.north_america.unwrap(),
            app_state.items.latin_america.unwrap(),
            2,
        );
        build_territory_connections(
            app_state,
            &mut connections,
            app_state.items.north_america.unwrap(),
            app_state.items.europe.unwrap(),
            1,
        );
        build_territory_connections(
            app_state,
            &mut connections,
            app_state.items.europe.unwrap(),
            app_state.items.eastern_europe.unwrap(),
            2,
        );
        build_territory_connections(
            app_state,
            &mut connections,
            app_state.items.europe.unwrap(),
            app_state.items.middle_east.unwrap(),
            2,
        );
        build_territory_connections(
            app_state,
            &mut connections,
            app_state.items.africa.unwrap(),
            app_state.items.middle_east.unwrap(),
            2,
        );
        build_territory_connections(
            app_state,
            &mut connections,
            app_state.items.africa.unwrap(),
            app_state.items.latin_america.unwrap(),
            1,
        );
        build_territory_connections(
            app_state,
            &mut connections,
            app_state.items.asia.unwrap(),
            app_state.items.middle_east.unwrap(),
            2,
        );
        build_territory_connections(
            app_state,
            &mut connections,
            app_state.items.asia.unwrap(),
            app_state.items.australia.unwrap(),
            1,
        );
        build_territory_connections(
            app_state,
            &mut connections,
            app_state.items.asia.unwrap(),
            app_state.items.eastern_europe.unwrap(),
            2,
        );
    }

    // Do actual connection creation
    for connection in connections {
        app_state.world.create_entity().with(connection).build();
    }
}
