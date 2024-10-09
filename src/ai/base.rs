use std::thread;
use crate::ai::tree_search::TreeSearchAI;
use crate::app_state::AppState;
use crate::game_state::GameState;

pub trait AI {
    fn new(strength: AIStrength, game_state: GameState) -> Self
    where
        Self: Sized;
    fn start(&self) {}
    
    fn next_move(&self) {
        println!("Next move");
    }
}

pub enum AIModel {
    TreeSearch,
}

pub enum AIStrength {
    Easy,
    Normal,
    Hard,
}

pub fn do_computer_turn(ai: Box<dyn AI>) {
    ai.start();
}

pub fn computer_turn(model: AIModel, strength: AIStrength, app_state: &AppState) {
    let game_state = GameState {
        player: app_state.current_turn.clone(),
        num_of_players: 0,
        territories: app_state.items.territories.clone(),
    };

    // Create AI model
    let ai = match model {
        AIModel::TreeSearch => TreeSearchAI::new(strength, game_state),
    };

    // Move the data into the closure
    let handle = thread::spawn(move || {
        do_computer_turn(Box::new(ai));
    });

    handle.join().unwrap();
}
