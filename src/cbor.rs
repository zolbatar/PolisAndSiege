use ciborium::de::from_reader;
use ciborium::Value;
use crate::model::city::City;
use crate::model::location::Location;
use crate::model::territory::Territory;
use crate::model::territory_polygon::TerritoryPolygon;

static REGIONS_CBOR: &[u8] = include_bytes!("../assets/Regions.cbor");

pub struct Cbor {
    territories: Vec<Territory>,
}

impl Cbor {
    pub fn new() -> Self {

        // Open file
        let reader = from_reader::<Value, _>(REGIONS_CBOR).expect("Can't load CBOR file");

        // Top level is a map of territories
        let mut territories = Vec::new();
        for (territory_name, territory_polygons_cities) in reader.as_map().expect("CBOR: Expecting map of territories") {
            println!("CBOR: Territory: {}", territory_name.as_text().unwrap());

            // Array of polygons, then cities
            let polygons_cities = territory_polygons_cities.as_array().expect("CBOR: Expected arrays of polygons and cities");

            // Array of polygons
            let mut territory = Territory::new(territory_name.as_text().unwrap().to_string());
            for polygon in polygons_cities[0].as_array().expect("CBOR: Expecting array of polygons")
            {
                // Each polygon is an array of points
                let mut territory_polygon = TerritoryPolygon::new();
                for latitude_longitude in polygon.as_array().expect("CBOR: Expecting array of points") {
                    let longitude = latitude_longitude.as_array().unwrap()[0].as_float().unwrap();
                    let latitude = latitude_longitude.as_array().unwrap()[1].as_float().unwrap();
                    if longitude >= -172.44 && longitude != 0.0 && latitude != 0.0 {
                        territory_polygon.locations.push(Location::new(longitude as f32, latitude as f32));
                    }
                }
                territory.polygons.push(territory_polygon);
            }

            // Array of cities
            for city in polygons_cities[1].as_array().expect("CBOR: Expecting array of cities")
            {
                let city_details = city.as_array().expect("CBOR: Expected an array of city details");
                let name = city_details[0].as_text().unwrap();
                let latitude = city_details[1].as_float().unwrap();
                let longitude = city_details[2].as_float().unwrap();
                let population: i64 = city_details[3].as_integer().unwrap().try_into().unwrap();
                territory.cities.push(City::new(name.to_string(), latitude as f32, longitude as f32, population));
            }
            territories.push(territory);
        }

        Cbor {
            territories
        }
    }
}
