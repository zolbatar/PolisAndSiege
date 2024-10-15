use crate::ai::game_state::GameState;
use crate::ai::possible_move::possible_moves;
use crate::ai::temp_player::TempPlayer;
use crate::app_state::AppState;
use crate::model::player::Player;
use crate::model::territory::Territory;
use specs::WorldExt;

pub fn computer_turn(app_state: &mut AppState) {
    {
        let players = app_state.world.read_storage::<Player>();
        let player = players.get(app_state.current_player).unwrap();
        print!("Starting score: {}, ", player.score);
    }

    // Collate a list of all cities
    let mut all_cities = Vec::new();
    {
        let territories = app_state.world.read_storage::<Territory>();
        for territory_entity in app_state.items.territories.values() {
            let territory = territories.get(*territory_entity).unwrap();
            for city_state in &territory.cities {
                all_cities.push(city_state.clone());
            }
        }
    }

    let mut temp_players = Vec::new();
    {
        let players = app_state.world.read_storage::<Player>();
        for player_entity in &app_state.players {
            let player = players.get(*player_entity).unwrap();
            temp_players.push(TempPlayer {
                armies_to_assign: player.armies_to_assign,
            })
        }
    }

    // Create initial game state
    let game_state = GameState {
        score: 0,
        current_player: Some(app_state.current_player),
        players: temp_players,
        mode: app_state.mode.clone(),
        depth: 0,
        city_states: all_cities,
        no_choices: 3,
    };

    let mut possibles = possible_moves(&game_state, app_state);
    if possibles.is_empty() {
        println!("no possible moves");
        return;
    } else {
        print!("there are {} possible moves, ", possibles.len());
    }

    // Score range
    let lowest = possibles.iter().min_by_key(|p| p.best_score).unwrap().best_score;
    let highest = possibles.iter().max_by_key(|p| p.best_score).unwrap().best_score;
    println!("lowest and highest score: {}/{}", lowest, highest);

    // Select move
    possibles.sort_by(|a, b| a.best_score.cmp(&b.best_score));
    let best = &mut possibles[0];
    best.do_move_and_next_turn(app_state);
    //println!("{:#?}", possibles);
}

pub fn _move_to_next_player(game_state: &mut GameState, app_state: &AppState, to_index: usize) -> bool {
    let players = app_state.world.read_storage::<Player>();
    let current_player = players.get(game_state.current_player.unwrap());

    // Next index
    let mut index = current_player.unwrap().index + 1;
    if index == app_state.num_of_players {
        index = 0;
    }
    game_state.current_player = Some(app_state.players[index]);
    index == to_index
}
