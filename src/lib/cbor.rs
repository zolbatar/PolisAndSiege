use crate::app_state::AppState;
use crate::model::city::{select_evenly_spaced_cities, City, CityTemporary};
use crate::model::connection::build_connections;
use crate::model::location::{Location};
use crate::model::territory::{get_colour_for_territory_name, Territory};
use crate::model::territory_polygon::{TerritoryPolygon};
use ciborium::de::from_reader;
use ciborium::Value;
use petgraph::prelude::NodeIndex;
use rand::seq::SliceRandom;
use rand::thread_rng;
use specs::{Builder, Entity, WorldExt};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use crate::model::city_state::CityState;

const REGIONS_CBOR: &[u8] = include_bytes!("../../assets/Regions.cbor");

pub fn import(app_state: &mut AppState) -> BTreeMap<String, Entity> {
    // Open file
    let reader = from_reader::<Value, _>(REGIONS_CBOR).expect("Can't load CBOR file");
    let mut polygon_count = 0;
    let mut point_count_total = 0;
    let mut cities_count = 0;

    // Top level is a map of territories
    let mut territories = BTreeMap::new();
    for (territory_name, territory_polygons_cities) in reader.as_map().expect("CBOR: Expecting map of territories") {
        let mut territory_name_unwrapped = territory_name.as_text().unwrap().to_string();

        // Rename if needed
        if territory_name_unwrapped == "Northern America" {
            territory_name_unwrapped = String::from("North America");
        }
        if territory_name_unwrapped == "Middle East & North Africa" {
            territory_name_unwrapped = String::from("Middle East and North Africa");
        }

        println!("CBOR: Territory '{}'", &territory_name_unwrapped);

        let _territory = app_state
            .world
            .create_entity()
            .with(Territory {
                colour: get_colour_for_territory_name(&territory_name_unwrapped),
                name: territory_name_unwrapped.clone(),
                ..Default::default()
            })
            .build();

        // Array of polygons, then cities
        let polygons_cities =
            territory_polygons_cities.as_array().expect("CBOR: Expected arrays of polygons and cities");

        // Array of polygons
        for polygon in polygons_cities[0].as_array().expect("CBOR: Expecting array of polygons") {
            // Each polygon is an array of points
            let mut _locations = Vec::new();
            let mut adjust_russia = false;
            for latitude_longitude in polygon.as_array().expect("CBOR: Expecting array of points") {
                let longitude = latitude_longitude.as_array().unwrap()[0].as_float().unwrap() as f32;
                let latitude = -latitude_longitude.as_array().unwrap()[1].as_float().unwrap() as f32;
                if longitude != 0.0 && latitude != 0.0 {
                    if longitude < -172.44 {
                        adjust_russia = true;
                    }

                    _locations.push(Location::new(longitude, latitude));
                    point_count_total += 1;
                }
            }

            // Adjust part of russia
            if adjust_russia {
                for location in &mut _locations {
                    location.longitude += 360.0;
                    location.p.x += 360.0;
                }
            }

            // We only want polygons of a minimum complexity
            if _locations.len() >= 64 {
                polygon_count += 1;

                let territory_polygon = TerritoryPolygon::new(
                    app_state,
                    _territory,
                    _locations);

                // Add polygon to territory
                app_state
                    .world
                    .write_storage::<Territory>()
                    .get_mut(_territory)
                    .unwrap()
                    .polygons
                    .push(territory_polygon);
            }
        }

        // Cities
        let mut territory_city = HashMap::new(); // Used as temporary storage before choosing cities
        for city in polygons_cities[1].as_array().expect("CBOR: Expecting array of cities") {
            let city_details = city.as_array().expect("CBOR: Expected an array of city details");
            let name = city_details[0].as_text().unwrap();
            let latitude = -city_details[1].as_float().unwrap() as f32;
            let longitude = city_details[2].as_float().unwrap() as f32;
            let population: i64 = city_details[3].as_integer().unwrap().try_into().unwrap();
            if !name.eq("Honolulu") && longitude > -140.0 {
                let city = CityTemporary::new(name.to_string(), longitude, latitude, population);
                if !territory_city.contains_key(&territory_name_unwrapped) {
                    territory_city.insert(territory_name_unwrapped.clone(), Vec::new());
                }
                territory_city.get_mut(&territory_name_unwrapped).unwrap().push(city);
                cities_count += 1;
            }
        }

        // Choose sensible cities for each territory
        territory_city = select_evenly_spaced_cities(app_state, &mut territory_city, 25);

        // Now add to ECS
        for (_, cities) in territory_city {
            for city in cities {
                let size = match city.population {
                    0..150000 => 1,
                    150000..500000 => 2,
                    500000..2500000 => 3,
                    2500000..5000000 => 4,
                    _ => 5,
                };

                let city = app_state
                    .world
                    .create_entity()
                    .with(City {
                        territory: _territory,
                        location: Location::new(city.location.longitude, city.location.latitude),
                        name: city.name,
                        size,
                        node: NodeIndex::new(0),
                    })
                    .build();

                let city_state = CityState {
                    city,
                    armies: 1,
                    owner: None,
                };

                // Add city to territory
                app_state.world.write_storage::<Territory>().get_mut(_territory).unwrap().cities
                    .push(Arc::new(Mutex::new(city_state)));
            }
        }

        territories.insert(territory_name_unwrapped, _territory);
    }

    app_state.items.north_america = Some(*territories.get("North America").unwrap());
    app_state.items.latin_america = Some(*territories.get("Latin America and the Caribbean").unwrap());
    app_state.items.europe = Some(*territories.get("Europe").unwrap());
    app_state.items.middle_east = Some(*territories.get("Middle East and North Africa").unwrap());
    app_state.items.eastern_europe = Some(*territories.get("Eastern Europe").unwrap());
    app_state.items.africa = Some(*territories.get("Sub-Saharan Africa").unwrap());
    app_state.items.asia = Some(*territories.get("Asia").unwrap());
    app_state.items.australia = Some(*territories.get("Australia and New Zealand").unwrap());

    // Now build connections
    build_connections(app_state);
    /*for connection in app_state.items.connections.iter() {
        let conn = connection.lock().unwrap();
        conn.city1.lock().unwrap().connections.push(connection.clone());
        conn.city2.lock().unwrap().connections.push(connection.clone());
    }*/

    // And a list of all cities
    let _territories = app_state.world.read_storage::<Territory>();
    for territory_entity in territories.values() {
        let territory = _territories.get(*territory_entity).unwrap();
        for city in territory.cities.iter() {
            app_state.items.cities.push(city.clone());
            app_state.items.cities_remaining_to_assign.push(city.clone());
        }
    }

    // Shuffle remaining ones randomly
    let mut rng = thread_rng(); // Create a random number generator
    app_state.items.cities_remaining_to_assign.shuffle(&mut rng); // Shuffle the vector in place

    println!("CBOR: Total territories: {}", territories.len());
    println!("CBOR: Total polygons: {}", polygon_count);
    println!("CBOR: Total points: {}", point_count_total);
    println!("CBOR: Total cities: {}", cities_count);

    territories
}
