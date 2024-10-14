use crate::ai::game_state::GameState;
use crate::ai::possible_move::possible_moves;
use crate::ai::temp_player::TempPlayer;
use crate::app_state::AppState;
use crate::model::player::Player;
use crate::model::territory::Territory;
use specs::WorldExt;

pub fn computer_turn(app_state: &mut AppState) {
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
                index: player.index,
                armies_to_assign: player.armies_to_assign,
            })
        }
    }

    // Create initial game state
    let mut game_state = GameState {
        score: 0,
        actual_human: Some(app_state.actual_human),
        current_turn: Some(app_state.current_player),
        players: temp_players,
        mode: app_state.mode.clone(),
        depth: 0,
        requested_depth: 0,
        city_states: all_cities,
    };
    game_state.calculate_score(app_state);
    game_state.display_score();

    let mut possibles = possible_moves(&game_state, app_state);
    if possibles.is_empty() {
        println!("No possible moves");
        return;
    }

    // Score range
    let lowest = possibles.iter().min_by_key(|p| p.game_state.score).unwrap().game_state.score;
    let highest = possibles.iter().min_by_key(|p| p.game_state.score).unwrap().game_state.score;
    println!("Lowest and highest score: {}/{}", lowest, highest);

    // Select move
    possibles.sort_by(|a, b| a.game_state.score.cmp(&b.game_state.score));
    let best = &mut possibles[0];
    best.do_move_and_next_turn(app_state);
    //    println!("{:?}", possibles);
}
