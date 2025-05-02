use crate::app_state::AppState;
use crate::lib::skia::Skia;
use crate::model::city::{select_evenly_spaced_cities, City};
use crate::model::connection::build_connections;
use crate::model::location::Location;
use crate::model::territory::{get_colour_for_territory_name, Territory};
use crate::model::territory_polygon::TerritoryPolygon;
use ciborium::de::from_reader;
use ciborium::Value;
use rand::rng;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

const REGIONS_CBOR: &[u8] = include_bytes!("../../assets/Regions.cbor");

pub fn import(skia: &mut Skia, app_state: &mut AppState) {
    // Open file
    let reader = from_reader::<Value, _>(REGIONS_CBOR).expect("Can't load CBOR file");
    let mut polygon_count = 0;
    let mut point_count_total = 0;
    let mut cities_count = 0usize;

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

        print!("CBOR: Territory '{}', ", &territory_name_unwrapped);

        let mut territory = Territory {
            colour: get_colour_for_territory_name(&territory_name_unwrapped),
            name: territory_name_unwrapped.clone(),
            ..Default::default()
        };

        // Array of polygons, then cities
        let polygons_cities =
            territory_polygons_cities.as_array().expect("CBOR: Expected arrays of polygons and cities");

        // Array of polygons
        for polygon in polygons_cities[0].as_array().expect("CBOR: Expecting array of polygons") {
            // Each polygon is an array of points
            let mut locations = Vec::new();
            let mut adjust_russia = false;
            for latitude_longitude in polygon.as_array().expect("CBOR: Expecting array of points") {
                let longitude = latitude_longitude.as_array().unwrap()[0].as_float().unwrap() as f32;
                let latitude = -latitude_longitude.as_array().unwrap()[1].as_float().unwrap() as f32;
                if longitude != 0.0 && latitude != 0.0 {
                    if longitude < -172.44 {
                        adjust_russia = true;
                    }

                    locations.push(Location::new(longitude, latitude));
                    point_count_total += 1;
                }
            }

            // Adjust part of russia
            if adjust_russia {
                for location in &mut locations {
                    location.longitude += 360.0;
                    location.p.x += 360.0;
                }
            }

            // We only want polygons of a minimum complexity
            if locations.len() >= 64 {
                polygon_count += 1;

                let territory_polygon = TerritoryPolygon::new(skia, territory.colour, locations);
                territory.polygons.push(territory_polygon);
            }
        }
        print!("{} polygons, ", territory.polygons.len());

        // Cities
        for city in polygons_cities[1].as_array().expect("CBOR: Expecting array of cities") {
            let city_details = city.as_array().expect("CBOR: Expected an array of city details");
            let name = city_details[0].as_text().unwrap();
            let latitude = -city_details[1].as_float().unwrap() as f32;
            let longitude = city_details[2].as_float().unwrap() as f32;
            let population: i64 = city_details[3].as_integer().unwrap().try_into().unwrap();
            if !name.eq("Honolulu") && longitude > -140.0 {
                let city = City::new(name.to_string(), longitude, latitude, population, territory.name.clone());
                territory.cities.push(Rc::new(RefCell::new(city)));
                cities_count += 1;
            }
        }
        println!("{} cities", territory.cities.len());

        territories.insert(territory_name_unwrapped, territory);
    }

    // Choose sensible cities for each territory
    select_evenly_spaced_cities(app_state, 25, territories);
    println!("Cities have been selected");

    // Build connections
    build_connections(&app_state.world_state, &mut app_state.world_fixed);
    println!("Connections have been built");

    // And a list of all cities
    for territory in app_state.world_fixed.territories.values() {
        for city in territory.cities.iter() {
            app_state.world_state.cities.push(city.clone());
            app_state.world_fixed.cities_to_assign.push(city.clone());
        }
    }

    // Shuffle remaining ones randomly
    let mut rng = rng(); // Create a random number generator
    app_state.world_fixed.cities_to_assign.shuffle(&mut rng); // Shuffle the vector in place

    println!("CBOR: Total territories: {}", app_state.world_fixed.territories.len());
    println!("CBOR: Total polygons: {}", polygon_count);
    println!("CBOR: Total points: {}", point_count_total);
    println!("CBOR: Total cities: {}", cities_count);
    println!("CBOR: Total cities used: {}", app_state.world_state.cities.len());
}
