use crate::lib::skia::Skia;
use crate::model::city::City;
use crate::model::location::Location;
use crate::model::territory::Territory;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::UnGraph;
use skia_safe::{dash_path_effect, Color, Paint, PaintStyle};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

const LINE_WIDTH: f32 = 0.2;

#[derive(Clone)]
pub struct Connection {
    paint: Paint,
    paint_alt: Paint,
    city1: Arc<Mutex<City>>,
    city2: Arc<Mutex<City>>,
}

impl Connection {
    pub fn new(city1: Arc<Mutex<City>>, city2: Arc<Mutex<City>>) -> Connection {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(Color::WHITE);
        paint.set_stroke_width(LINE_WIDTH);
        paint.set_style(PaintStyle::Stroke);
        let mut paint_alt = Paint::default();
        paint_alt.set_anti_alias(true);
        paint_alt.set_color(Color::BLACK);
        paint_alt.set_stroke_width(LINE_WIDTH);
        paint_alt.set_style(PaintStyle::Stroke);
        Connection {
            paint,
            paint_alt,
            city1: Arc::clone(&city1),
            city2: Arc::clone(&city2),
        }
    }

    pub fn render(&mut self, skia: &mut Skia, phase: f32) {
        self.paint.set_path_effect(dash_path_effect::new(&[1.0, 1.0], phase).unwrap());
        self.paint_alt.set_path_effect(dash_path_effect::new(&[1.0, 1.0], phase + 1.0).unwrap());
        skia.get_canvas().draw_line(
            self.city1.lock().unwrap().location.p,
            self.city2.lock().unwrap().location.p,
            &self.paint,
        );
        skia.get_canvas().draw_line(
            self.city1.lock().unwrap().location.p,
            self.city2.lock().unwrap().location.p,
            &self.paint_alt,
        );
    }
}

fn build_territory_connections(
    connections: &mut Vec<Connection>,
    territory1: Arc<Mutex<Territory>>,
    territory2: Arc<Mutex<Territory>>,
    num_connections: usize,
) {
    let mut m = BTreeMap::new();

    let cities1 = territory1.lock().unwrap().cities.clone();
    let cities2 = territory2.lock().unwrap().cities.clone();
    for city1 in cities1.iter() {
        for city2 in cities2.iter() {
            let distance =
                Location::calculate_distance(&city1.lock().unwrap().location, &city2.lock().unwrap().location) * 1000.0;
            m.insert(distance as usize, Connection::new(city1.clone(), city2.clone()));
        }
    }

    if m.is_empty() {
        panic!("Not enough connections");
    }

    // Shrink and return
    let mut iter = m.iter();
    for _ in 0..num_connections {
        let v = iter.next().unwrap();
        connections.push(v.1.clone());
    }
}

pub fn build_connections(territories: &BTreeMap<String, Arc<Mutex<Territory>>>) -> Vec<Connection> {
    let mut connections = Vec::new();
    for territory in territories {
        let mut graph = UnGraph::new_undirected();
        let cities = &territory.1.lock().unwrap().cities;

        // Cities
        for city in cities {
            let node = graph.add_node(city.clone());
            city.lock().unwrap().node = node;
        }

        // Distances
        for city_first in cities {
            for city_second in cities {
                if !Arc::ptr_eq(city_first, city_second) {
                    let distance = City::calculate_distance(&city_first.lock().unwrap(), &city_second.lock().unwrap());
                    graph.add_edge(city_first.lock().unwrap().node, city_second.lock().unwrap().node, distance);
                }
            }
        }

        // Get connections
        let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&graph));
        for edge in mst.raw_edges() {
            let _weight = edge.weight;
            let source = graph[edge.source()].clone();
            let target = graph[edge.target()].clone();
            connections.push(Connection::new(source.clone(), target.clone()));
        }
    }

    // Now we need inter-territory connections
    build_territory_connections(
        &mut connections,
        territories.get("North America").unwrap().clone(),
        territories.get("Latin America and the Caribbean").unwrap().clone(),
        2,
    );
    build_territory_connections(
        &mut connections,
        territories.get("North America").unwrap().clone(),
        territories.get("Europe").unwrap().clone(),
        1,
    );
    build_territory_connections(
        &mut connections,
        territories.get("Europe").unwrap().clone(),
        territories.get("Eastern Europe").unwrap().clone(),
        2,
    );
    build_territory_connections(
        &mut connections,
        territories.get("Europe").unwrap().clone(),
        territories.get("Middle East and North Africa").unwrap().clone(),
        2,
    );
    build_territory_connections(
        &mut connections,
        territories.get("Sub-Saharan Africa").unwrap().clone(),
        territories.get("Middle East and North Africa").unwrap().clone(),
        2,
    );
    build_territory_connections(
        &mut connections,
        territories.get("Sub-Saharan Africa").unwrap().clone(),
        territories.get("Latin America and the Caribbean").unwrap().clone(),
        1,
    );
    build_territory_connections(
        &mut connections,
        territories.get("Asia").unwrap().clone(),
        territories.get("Middle East and North Africa").unwrap().clone(),
        2,
    );
    build_territory_connections(
        &mut connections,
        territories.get("Asia").unwrap().clone(),
        territories.get("Australia and New Zealand").unwrap().clone(),
        1,
    );
    build_territory_connections(
        &mut connections,
        territories.get("Asia").unwrap().clone(),
        territories.get("Eastern Europe").unwrap().clone(),
        2,
    );

    connections
}
