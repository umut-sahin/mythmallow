use crate::prelude::*;


/// Starts loading the game.
pub fn start_loading(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Loading);
}

/// Starts the game.
pub fn start_playing(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Playing);
}

/// Restarts the game.
pub fn restart(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Initialization);
}
