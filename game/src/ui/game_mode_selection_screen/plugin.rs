use crate::{
    prelude::*,
    ui::game_mode_selection_screen::systems::*,
};

/// Plugin for managing the game mode selection screen.
pub struct GameModeSelectionScreenPlugin;

impl Plugin for GameModeSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<GameModeSelectionScreen>();

        // Add systems.
        app.add_systems(
            OnEnter(AppState::GameModeSelectionScreen),
            spawn_game_mode_selection_screen,
        );
        app.add_systems(
            PostUpdate,
            game_mode_selected.in_set(GameModeSelectionScreenSystems).run_if(
                |game_mode_index: Option<Res<SelectedGameModeIndex>>| game_mode_index.is_some(),
            ),
        );
        app.add_systems(
            OnExit(AppState::GameModeSelectionScreen),
            despawn_game_mode_selection_screen,
        );

        // Select the game mode when starting in game.
        let args = app.world_mut().resource::<Args>();
        if args.start_in_game {
            app.add_systems(
                OnEnter(AppState::GameModeSelectionScreen),
                select_game_mode_when_starting_in_game.run_if(run_once()),
            );
        }
    }
}
