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

        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("ui/game_mode_selection_screen.ftl");

        // Add systems.
        app.add_systems(
            OnEnter(AppState::GameModeSelectionScreen),
            spawn_game_mode_selection_screen,
        );
        app.add_systems(PreUpdate, navigation.in_set(GameModeSelectionScreenSystems));
        app.add_systems(
            Update,
            (game_mode_button_interaction, back_button_interaction)
                .in_set(GameModeSelectionScreenSystems),
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
