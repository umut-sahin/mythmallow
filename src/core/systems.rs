use crate::prelude::*;


/// Restarts the game.
pub fn restart(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_app_state.set(AppState::Game);
    next_game_state.set(GameState::Playing);
}
