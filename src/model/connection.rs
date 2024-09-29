use crate::model::city::City;
use crate::model::territory::Territory;
use petgraph::graph::UnGraph;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use skia_safe::{dash_path_effect, paint, scalar, Canvas, Paint, PaintStyle, PathEffect};

pub struct Connection {
    paint: Paint,
    city1: Arc<Mutex<City>>,
    city2: Arc<Mutex<City>>,
}

impl Connection {
    pub fn new(city1: Arc<Mutex<City>>, city2: Arc<Mutex<City>>) -> Connection {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_argb(255, 20, 20, 20);
        paint.set_stroke_width(0.25);
        paint.set_style(PaintStyle::Stroke);
        let dash = dash_path_effect::new(&[1.0, 0.5], 0.0).unwrap();
        //paint.set_path_effect(dash);
        Connection {
            paint,
            city1: Arc::clone(&city1),
            city2: Arc::clone(&city2),
        }
    }

    pub fn render(&mut self, canvas: &Canvas) {
        canvas.draw_line(self.city1.lock().unwrap().location.p, self.city2.lock().unwrap().location.p, &self.paint);
    }
}

pub fn build_connections(territories: &HashMap<String, Territory>) -> Vec<Connection> {
    let mut connections = Vec::new();
    for territory in territories {
        let mut graph = UnGraph::new_undirected();

        // Cities
        for city in &territory.1.cities {
            let node = graph.add_node(city);
            city.lock().unwrap().node = node;
        }

        // Distances
        for city_first in &territory.1.cities {
            for city_second in &territory.1.cities {
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
            let source = graph[edge.source()];
            let target = graph[edge.target()];
            connections.push(Connection::new(source.clone(), target.clone()));
        }
    }
    connections
}