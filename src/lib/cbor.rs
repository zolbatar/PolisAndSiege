use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ciborium::de::from_reader;
use ciborium::Value;
use crate::app_state::AppState;
use crate::model::city::City;
use crate::model::connection::build_connections;
use crate::model::location::Location;
use crate::model::territory::Territory;
use crate::model::territory_polygon::TerritoryPolygon;

const REGIONS_CBOR: &[u8] = include_bytes!("../../assets/Regions.cbor");

pub fn import(app_state: &mut AppState) -> HashMap<String, Arc<Mutex<Territory>>> {

    // Open file
    let reader = from_reader::<Value, _>(REGIONS_CBOR).expect("Can't load CBOR file");
    let mut polygon_count = 0;
    let mut point_count_total = 0;
    let mut cities_count = 0;

    // Top level is a map of territories
    let mut territories = HashMap::new();
    for (territory_name, territory_polygons_cities) in reader.as_map().expect("CBOR: Expecting map of territories") {
        let mut territory_name_unwrapped = territory_name.as_text().unwrap().to_string();

        // Rename if needed
        if territory_name_unwrapped == "Northern America" { territory_name_unwrapped = String::from("North America"); }
        if territory_name_unwrapped == "Middle East & North Africa" { territory_name_unwrapped = String::from("Middle East and North Africa"); }

        println!("CBOR: Territory '{}'", &territory_name_unwrapped);

        // Array of polygons, then cities
        let polygons_cities = territory_polygons_cities.as_array().expect("CBOR: Expected arrays of polygons and cities");

        // Array of polygons
        let territory = Arc::new(Mutex::new(Territory::new(&territory_name_unwrapped)));
        for polygon in polygons_cities[0].as_array().expect("CBOR: Expecting array of polygons")
        {
            // Each polygon is an array of points
            let mut territory_polygon = TerritoryPolygon::new();
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
                for location in locations.iter_mut() {
                    location.p.x += 360.0;
                }
            }
            territory_polygon.locations.append(&mut locations);

            if territory_polygon.locations.len() >= 64 {
                territory.lock().unwrap().polygons.push(territory_polygon);
                polygon_count += 1;
            }
        }

        let _ = territory.lock().unwrap().prerender_polygons();

        // Cities
        for city in polygons_cities[1].as_array().expect("CBOR: Expecting array of cities")
        {
            let city_details = city.as_array().expect("CBOR: Expected an array of city details");
            let name = city_details[0].as_text().unwrap();
            let latitude = -city_details[1].as_float().unwrap() as f32;
            let longitude = city_details[2].as_float().unwrap() as f32;
            let population: i64 = city_details[3].as_integer().unwrap().try_into().unwrap();
            if !name.eq("Honolulu") && longitude > -140.0 {
                let city = Arc::new(Mutex::new(City::new(name.to_string(), longitude, latitude, population, territory.clone())));
                territory.lock().unwrap().cities.push(city);
                cities_count += 1;
            }
        }

        // Choose sensible cities
        let t_clone = territory.clone();
        territory.lock().unwrap().cities = City::select_evenly_spaced_cities(app_state, t_clone, 25);

        territories.insert(territory_name_unwrapped, territory);
    }

    // Now build connections
    app_state.connections = build_connections(&territories);

    // And a list of all cities
    for territory in territories.values() {
        for city in territory.lock().unwrap().cities.iter() {
            app_state.cities.push(city.clone());
        }
    }

    println!("CBOR: Total territories: {}", territories.len());
    println!("CBOR: Total polygons: {}", polygon_count);
    println!("CBOR: Total points: {}", point_count_total);
    println!("CBOR: Total cities: {}", cities_count);

    territories
}
