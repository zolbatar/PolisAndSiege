use crate::ai::tree_search::TreeSearchAI;
use crate::game_state::GameState;
use crate::model::city::Owner;

pub trait AI {
    fn new(strength: AIStrength, game_state: GameState) -> Self;
    fn next_move(&self);
}

pub enum AIModel {
    Rules,
}

pub enum AIStrength {
    Easy,
    Normal,
    Hard,
}

pub fn computer_turn(model: AIModel, strength: AIStrength) {
    let game_state = GameState {
        player: Owner::None,
        num_of_players: 0,
    };
    let ai = match model {
        AIModel::Rules => TreeSearchAI::new(strength, game_state),
    };
}
