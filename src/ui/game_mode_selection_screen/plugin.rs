use crate::{
    prelude::*,
    ui::game_mode_selection_screen::systems::*,
};

/// Plugin for managing the game mode selection screen.
pub struct GameModeSelectionScreenPlugin;

impl Plugin for GameModeSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameModeSelectionScreen>();

        app.add_systems(
            OnEnter(AppState::GameModeSelectionScreen),
            spawn_game_mode_selection_screen,
        );
        app.add_systems(
            OnExit(AppState::GameModeSelectionScreen),
            despawn_game_mode_selection_screen,
        );
    }
}
