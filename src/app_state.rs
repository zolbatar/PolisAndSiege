use crate::model::city_state::CityState;
use crate::model::location::Location;
use crate::model::player::Player;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use sdl2::video::Window;
use skia_safe::svg::Dom;
use skia_safe::{Color, FontMgr, Path, Point, Size};
use specs::{Builder, Entity, World, WorldExt};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const SVG_CORNER: &str = include_str!("../assets/Corner.svg");
const SVG_SIDE: &str = include_str!("../assets/Side.svg");
const SVG_BUTTON: &str = include_str!("../assets/Button.svg");
pub const NOISE_MIX: f32 = 0.075;
pub(crate) const MIN_ZOOM: f32 = 4.2;

#[derive(PartialEq, Clone, Default, Debug)]
pub enum GameMode {
    Randomising,
    ArmyPlacement,
    #[default]
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
    pub button_path: Dom,
    pub player_lookup: HashMap<usize, Entity>,
}

#[derive(Default)]
pub struct Items {
    pub territories: BTreeMap<String, Entity>,
    pub existing_cities: Vec<Location>, // Only used during initial city placement
    pub cities: Vec<Arc<Mutex<CityState>>>,
    pub cities_remaining_to_assign: Vec<Arc<Mutex<CityState>>>,
    pub north_america: Option<Entity>,
    pub latin_america: Option<Entity>,
    pub asia: Option<Entity>,
    pub europe: Option<Entity>,
    pub eastern_europe: Option<Entity>,
    pub australia: Option<Entity>,
    pub middle_east: Option<Entity>,
    pub africa: Option<Entity>,
}

pub struct CitySelection {
    pub last_selection: Instant,
    pub last_city_hover: Option<Arc<Mutex<CityState>>>,
    pub last_city_selection: Option<Arc<Mutex<CityState>>>,
    pub last_army_city_selection: Option<Arc<Mutex<CityState>>>,
    pub minimum_allowed_distance: f32,
    pub assign_speed: u128,
}

pub struct AppState {
    pub world: World,
    pub players: Vec<Entity>,
    pub num_of_players: usize,

    pub selection: CitySelection,
    pub gfx: GFXState,
    pub res: Resource,
    pub items: Items,
    pub mode: GameMode,
    pub fps: f64,
    pub zoom: f32,
    pub target: Point,
    pub panning: bool,
    pub show_labels: bool,
    pub show_shadows: bool,
    pub phase: f32,
    pub current_player: Entity,
    pub actual_human: Entity,
    pub hover: Point,
}

impl AppState {
    pub fn new(window: &Window, dpi: f32, mut world: World) -> Self {
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
        let button_path = Dom::from_str(SVG_BUTTON, FontMgr::new()).expect("Error loading SVG");

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
            button_path,
            player_lookup: HashMap::new(),
        };

        let num_of_players = 2;

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
            "The Iberian Dominion",
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

        // Player colours
        let player_colours = [
            vec![Color::from_rgb(128, 128, 255), Color::BLACK],
            vec![Color::from_rgb(255, 128, 128), Color::BLACK],
            vec![Color::from_rgb(128, 255, 128), Color::BLACK],
            vec![Color::from_rgb(255, 255, 128), Color::BLACK],
            vec![Color::from_rgb(128, 255, 255), Color::BLACK],
        ];

        // Create player(s)
        let mut _players = Vec::new();
        for i in 0..num_of_players {
            _players.push(
                world
                    .create_entity()
                    .with(Player {
                        index: i,
                        name: possible_names[i].parse().unwrap(),
                        colours: player_colours[i].clone(),
                        armies_to_assign: 10,
                        ..Default::default()
                    })
                    .build(),
            );
        }

        // Colours for each player
        res.player_lookup.insert(0, _players[0]);
        res.player_lookup.insert(1, _players[1]);
        if num_of_players >= 3 {
            res.player_lookup.insert(2, _players[2]);
        }
        if num_of_players >= 4 {
            res.player_lookup.insert(3, _players[3]);
        }
        if num_of_players >= 5 {
            res.player_lookup.insert(4, _players[4]);
        }

        let items = Items::default();

        AppState {
            world,
            mode: GameMode::Randomising,
            selection: CitySelection {
                last_selection: Instant::now(),
                last_city_selection: None,
                last_city_hover: None,
                last_army_city_selection: None,
                minimum_allowed_distance: 40.0, //18.0, //12.0,
                assign_speed: 0,
            },
            gfx,
            res,
            items,
            current_player: _players[0],
            actual_human: _players[0],
            players: _players,
            num_of_players,
            fps: 0.0,
            panning: false,
            show_labels: true,
            show_shadows: true,
            zoom: MIN_ZOOM,
            target: Point::new(25.0, -10.0),
            phase: 0.0,
            hover: Point::new(-1.0, -1.0),
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
