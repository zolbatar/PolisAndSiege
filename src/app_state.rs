use std::cell::RefCell;
use std::rc::Rc;
use crate::model::ai_profile::AIProfile;
use crate::model::player::Player;
use crate::model::world_fixed::WorldFixed;
use crate::model::world_state::WorldState;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use sdl2::video::Window;
use skia_safe::svg::Dom;
use skia_safe::{Color, FontMgr, Path, Point, Size};
use std::time::Instant;
use crate::model::city::CityRR;

const SVG_CORNER: &str = include_str!("../assets/Corner.svg");
const SVG_SIDE: &str = include_str!("../assets/Side.svg");
const SVG_BUTTON: &str = include_str!("../assets/Button.svg");
pub const NOISE_MIX: f32 = 0.075;
pub const MIN_ZOOM: f32 = 4.2;

#[derive(PartialEq, Clone, Default, Debug)]
pub enum GameMode {
    #[default]
    Randomising,
    ArmyPlacement,
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
}

#[derive(Debug)]
pub struct CitySelection {
    pub last_selection: Instant,
    pub last_city_hover: Option<CityRR>,
    pub last_city_selection: Option<CityRR>,
    pub last_army_city_selection: Option<CityRR>,
    pub minimum_allowed_distance: f32,
    pub assign_speed: u128,
}

pub struct AppState {
    pub world_state: WorldState,
    pub world_fixed: WorldFixed,
    pub num_of_players: usize,
    pub gfx: GFXState,
    pub res: Resource,
    pub fps: f64,
    pub show_labels: bool,
    pub show_shadows: bool,
    pub phase: f32,
    pub selection: CitySelection,
    pub hover: Point,
    pub target: Point,
    pub panning: bool,
    pub zoom: f32,
}

impl AppState {
    pub fn new(window: &Window, dpi: f32, mut world_state: WorldState) -> Self {
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

        let res = Resource {
            corner_path,
            side_path,
            button_path,
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

        let profile = AIProfile {
            human: true,
            no_choices: 3,
            search_depth: 3,
            city_size_multiplier: 1.5,
            army_multiplier: 1.0,
            army_same_territory: 2.0,
            army_bordering: 3.0,
            random_fraction: 0.1,
        };

        let ai_profile = AIProfile {
            human: false,
            no_choices: 3,
            search_depth: 3,
            city_size_multiplier: 1.5,
            army_multiplier: 1.0,
            army_same_territory: 2.0,
            army_bordering: 3.0,
            random_fraction: 0.1,
        };

        // Create player(s)
        for i in 0..num_of_players {
            let player = Player {
                index: i,
                name: possible_names[i].parse().unwrap(),
                colours: player_colours[i].clone(),
                armies_to_assign: 10,
                cities: Vec::new(),
                score: 0,
                profile: if i == 0 {
                    profile.clone()
                } else {
                    ai_profile.clone()
                },
            };
            world_state.players.push(Rc::new(RefCell::new(player)));
        }

        AppState {
            world_state,
            selection: CitySelection {
                last_selection: Instant::now(),
                last_city_selection: None,
                last_city_hover: None,
                last_army_city_selection: None,
                minimum_allowed_distance: 25.0, //18.0, //12.0,
                assign_speed: 0,
            },
            hover: Default::default(),
            target: Default::default(),
            panning: false,
            world_fixed: WorldFixed::default(),
            gfx,
            res,
            num_of_players,
            fps: 0.0,
            show_labels: true,
            show_shadows: true,
            phase: 0.0,
            zoom: MIN_ZOOM,
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
