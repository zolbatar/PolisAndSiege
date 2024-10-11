use crate::app_state::AppState;
use crate::model::city::CCity;
use crate::model::location::{calculate_distance_new, CLocation};
use crate::model::territory::CTerritory;
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
pub struct CConnection {
    pub city1: Entity,
    pub city2: Entity,
    pub render: bool,
}

fn build_territory_connections(
    app_state: &mut AppState,
    connections: &mut Vec<CConnection>,
    territory1_entity: Entity,
    territory2_entity: Entity,
    num_connections: usize,
) {
    let mut m1 = BTreeMap::new();
    let mut m2 = BTreeMap::new();
    let cities = app_state.world.read_storage::<CCity>();
    let locations = app_state.world.read_storage::<CLocation>();
    let territories = app_state.world.read_storage::<CTerritory>();

    let territory1 = territories.get(territory1_entity).unwrap();
    let territory2 = territories.get(territory2_entity).unwrap();
    for city1_entity in territory1.cities.iter() {
        for city2_entity in territory2.cities.iter() {
            let city1 = cities.get(*city1_entity).unwrap();
            let city2 = cities.get(*city2_entity).unwrap();
            if city1 != city2 {
                let city1_location = locations.get(city1.location).unwrap();
                let city2_location = locations.get(city2.location).unwrap();
                let distance = calculate_distance_new(city1_location, city2_location);
                m1.insert(
                    distance as usize,
                    CConnection {
                        city1: *city1_entity,
                        city2: *city2_entity,
                        render: true,
                    },
                );
                m2.insert(
                    distance as usize,
                    CConnection {
                        city2: *city1_entity,
                        city1: *city2_entity,
                        render: false,
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
    for territory in app_state.world.read_storage::<CTerritory>().join() {
        let mut graph = UnGraph::new_undirected();

        // Cities
        let mut cities = app_state.world.write_storage::<CCity>();
        for entity in &territory.cities {
            let city = cities.get_mut(*entity).unwrap();
            let node = graph.add_node(entity);
            city.node = node;
        }

        // Distances
        let locations = app_state.world.read_storage::<CLocation>();
        for city1_entity in &territory.cities {
            for city2_entity in &territory.cities {
                let city1 = cities.get(*city1_entity).unwrap();
                let city2 = cities.get(*city2_entity).unwrap();
                if city1 != city2 {
                    let city1_location = locations.get(city1.location).unwrap();
                    let city2_location = locations.get(city2.location).unwrap();
                    let distance = calculate_distance_new(city1_location, city2_location);
                    graph.add_edge(city1.node, city2.node, distance);
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
            connections.push(CConnection {
                city1: source,
                city2: target,
                render: true,
            });
            connections.push(CConnection {
                city2: source,
                city1: target,
                render: false,
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
