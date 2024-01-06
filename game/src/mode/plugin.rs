use crate::{
    mode::systems::*,
    prelude::*,
};

/// Plugin for managing game modes.
pub struct ModePlugin;

impl Plugin for ModePlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<SelectedGameModeIndex>();

        // Initialize registry.
        app.init_resource::<GameModeRegistry>();

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            initialize_game_mode.in_set(InitializationSystems::GameMode),
        );
        app.add_systems(
            OnEnter(GameState::Restart),
            restart_game_mode.in_set(RestartSystems::GameMode),
        );
        app.add_systems(OnExit(AppState::Game), deinitialize_game_mode);
    }
}
