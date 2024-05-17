use crate::prelude::*;


/// Starts loading the game.
pub fn start_loading(
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    game_state_stack.transition(GameState::Loading);
    next_game_state.set(GameState::Transition);
}

/// Starts the game.
pub fn start_playing(
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    game_state_stack.transition(GameState::Playing);
    next_game_state.set(GameState::Transition);
}

/// Restarts the game.
pub fn restart(
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    game_state_stack.transition(GameState::Initialization);
    next_game_state.set(GameState::Transition);
}


/// Transitions to top state in the game state stack when current game state is transition.
pub fn game_state_transition(
    game_state_stack: Res<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Some(game_state) = game_state_stack.last() {
        next_game_state.set(*game_state);
    }
}

/// Clears the game state stack.
pub fn reset_game_state_stack(
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    game_state_stack.clear();
    next_game_state.set(GameState::Transition);
}
