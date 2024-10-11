use crate::app_state::AppState;
use crate::game_state::GameState;
use specs::{DispatcherBuilder, System};

pub enum Difficulty {
    TreeSearchEasy,
    TreeSearchNormal,
    TreeSearchHard,
}

struct SRunComputerOpponent;
impl<'a> System<'a> for SRunComputerOpponent {
    type SystemData = specs::Read<'a, GameState>;

    fn run(&mut self, game_state: Self::SystemData) {}
}

pub fn computer_turn(app_state: &mut AppState) {
    // Create and insert game state
    let game_state = GameState {
        actual_human: Some(app_state.actual_human),
        current_turn: Some(app_state.current_turn),
        players: app_state.players.clone(),
        territories: app_state.items.territories.clone(),
        mode: app_state.mode.clone(),
    };
    app_state.world.insert(game_state);

    // Do it
    let mut dispatcher_ai = DispatcherBuilder::new().with(SRunComputerOpponent, "computer_oppenent", &[]).build();
    dispatcher_ai.dispatch_par(&app_state.world);
}
