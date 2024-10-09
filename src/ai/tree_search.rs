use crate::ai::base::{AIStrength, AI};
use crate::game_state::GameState;

pub struct TreeSearchAI {
    strength: AIStrength,
    game_state: GameState,
}

impl AI for TreeSearchAI {
    fn new(strength: AIStrength, game_state: GameState) -> Self {
        TreeSearchAI {
            strength,
            game_state,
        }
    }
}
