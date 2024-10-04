use crate::model::city::{City, Owner};
use crate::model::connection::Connection;
use crate::model::location::Location;
use crate::model::territory::Territory;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use sdl2::video::Window;
use skia_safe::svg::Dom;
use skia_safe::{Color, FontMgr, Path, Point, Size};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const SVG_CORNER: &str = include_str!("../assets/Corner.svg");
const SVG_SIDE: &str = include_str!("../assets/Side.svg");
pub const NOISE_MIX: f32 = 0.075;
pub(crate) const MIN_ZOOM: f32 = 4.2;

pub enum GameMode {
    Randomising,
    ArmyPlacement,
    CitySelection,
    Game,
}

pub struct GFXState {
    pub width: i32,
    pub height: i32,
    pub half_width: i32,
    pub half_height: i32,
    pub dpi: f32,
}

pub struct Resource {
    pub side_path: Dom,
    pub corner_path: Dom,
    pub player_lookup: HashMap<u8, Owner>,
    pub player_colours: HashMap<Owner, Vec<Color>>,
    pub player_name: HashMap<Owner, String>,
}

pub struct Items {
    pub territories: BTreeMap<String, Arc<Mutex<Territory>>>,
    pub existing_cities: Vec<Location>,
    pub cities: Vec<Arc<Mutex<City>>>,
    pub cities_remaining_to_assign: Vec<Arc<Mutex<City>>>,
    pub connections: Vec<Connection>,
}

pub struct CitySelection {
    pub last_selection: Instant,
    pub last_city_selection: Option<Arc<Mutex<City>>>,
    pub last_player: u8,
    pub minimum_allowed_distance: f32,
    pub assign_speed: u128,
}

pub struct AppState {
    pub selection: CitySelection,
    pub gfx: GFXState,
    pub res: Resource,
    pub items: Items,
    pub mode: GameMode,
    pub num_of_players: u8,
    pub fps: f64,
    pub zoom: f32,
    pub target: Point,
    pub panning: bool,
    pub show_labels: bool,
    pub show_shadows: bool,
    pub phase: f32,
    pub armies_to_assign: i32,
}

impl AppState {
    pub fn new(window: &Window, dpi: f32) -> Self {
        let width = window.size().0 as i32;
        let height = window.size().1 as i32;
        println!("Screen resolution: {}x{}", width, height);
        let half_width = width / 2;
        let half_height = height / 2;

        let st = "";
        Path::from_svg(st);
        let mut corner_path = Dom::from_str(SVG_CORNER, FontMgr::new()).expect("Error loading SVG");
        corner_path.set_container_size(Size::new(160.0, 160.0));
        let mut side_path = Dom::from_str(SVG_SIDE, FontMgr::new()).expect("Error loading SVG");
        side_path.set_container_size(Size::new(40.0, 200.0));

        let gfx = GFXState {
            width,
            height,
            half_width,
            half_height,
            dpi,
        };

        let mut res = Resource {
            corner_path,
            side_path,
            player_lookup: HashMap::new(),
            player_colours: HashMap::new(),
            player_name: HashMap::new(),
        };

        let mut possible_names = vec![
            "The Britannian Dominion",
            "The Red Tsardom",
            "The Iron Kaisers",
            "The Rising Shogunate",
            "The Gaulish Syndicate",
            "The Yankee Federation",
            "The Ottoman Remnants",
            "The Austro Imperium",
            "The Persian Ascendants",
            "The Italian Legions",
            "The Dragon Empire",
            " The Iberian Dominion",
            "The Nordic Coalition",
            "The Balkan Confederacy",
            "The Egyptian Dynasts",
            "The Prussian Order",
            "The Celtic Union",
            "The Maharaja Confederation",
            "The Andean Empire",
            "The Hellenic Guardians",
        ];
        let mut rng = thread_rng(); // Create a random number generator
        possible_names.shuffle(&mut rng);

        // Colours for each player
        res.player_lookup.insert(0, Owner::None);
        res.player_lookup.insert(1, Owner::Player);
        res.player_lookup.insert(2, Owner::Enemy1);
        res.player_lookup.insert(3, Owner::Enemy2);
        res.player_lookup.insert(4, Owner::Enemy3);
        res.player_lookup.insert(5, Owner::Enemy4);
        res.player_colours.insert(Owner::None, vec![Color::from_rgb(128, 128, 128), Color::BLACK]);
        res.player_colours.insert(Owner::Player, vec![Color::from_rgb(0, 0, 255), Color::WHITE]);
        res.player_colours.insert(Owner::Enemy1, vec![Color::from_rgb(255, 0, 0), Color::WHITE]);
        res.player_colours.insert(Owner::Enemy2, vec![Color::from_rgb(0, 255, 0), Color::BLACK]);
        res.player_colours.insert(Owner::Enemy3, vec![Color::from_rgb(255, 255, 0), Color::BLACK]);
        res.player_colours.insert(Owner::Enemy4, vec![Color::from_rgb(0, 255, 255), Color::BLACK]);
        res.player_name.insert(Owner::None, "No control".parse().unwrap());
        res.player_name.insert(Owner::Player, possible_names[0].parse().unwrap());
        res.player_name.insert(Owner::Enemy1, possible_names[1].parse().unwrap());
        res.player_name.insert(Owner::Enemy2, possible_names[2].parse().unwrap());
        res.player_name.insert(Owner::Enemy3, possible_names[3].parse().unwrap());
        res.player_name.insert(Owner::Enemy4, possible_names[4].parse().unwrap());

        let items = Items {
            territories: BTreeMap::new(),
            existing_cities: Vec::new(),
            connections: Vec::new(),
            cities: Vec::new(),
            cities_remaining_to_assign: Vec::new(),
        };

        AppState {
            mode: GameMode::Randomising,
            selection: CitySelection {
                last_selection: Instant::now(),
                last_city_selection: None,
                last_player: 1,
                minimum_allowed_distance: 18.0, //12.0,
                assign_speed: 1,
            },
            gfx,
            res,
            items,
            num_of_players: 5,
            fps: 0.0,
            panning: false,
            show_labels: true,
            show_shadows: true,
            zoom: MIN_ZOOM,
            target: Point::new(25.0, -10.0),
            phase: 0.0,
            armies_to_assign: 10,
        }
    }

    pub fn reset(&mut self) {
        self.zoom = MIN_ZOOM;
        self.target = Point::new(25.0, -10.0);
    }

    pub fn show_all_info(&self) -> bool {
        self.show_labels
    }
}
