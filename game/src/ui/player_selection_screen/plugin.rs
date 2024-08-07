use crate::{
    prelude::*,
    ui::player_selection_screen::systems::*,
};

/// Plugin for managing the player selection screen.
pub struct PlayerSelectionScreenPlugin;

impl Plugin for PlayerSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<PlayerSelectionScreen>();
        app.register_type::<PlayerSelectionScreenPlayerButton>();
        app.register_type::<PlayerSelectionScreenBackButton>();

        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("ui/player_selection_screen.ftl");

        // Add systems.
        app.add_systems(OnEnter(AppState::PlayerSelectionScreen), spawn_player_selection_screen);
        app.add_systems(Update, navigation.in_set(PlayerSelectionScreenSystems));
        app.add_systems(
            Update,
            (player_button_interaction, back_button_interaction)
                .in_set(PlayerSelectionScreenSystems),
        );
        app.add_systems(
            PostUpdate,
            player_selected.in_set(PlayerSelectionScreenSystems).run_if(
                |selected_mythology_index: Option<Res<SelectedMythologyIndex>>,
                 selected_player_index: Option<Res<SelectedPlayerIndex>>| {
                    selected_mythology_index.is_some() && selected_player_index.is_some()
                },
            ),
        );
        app.add_systems(OnExit(AppState::PlayerSelectionScreen), despawn_player_selection_screen);

        // Select the player when starting in game.
        let args = app.world_mut().resource::<Args>();
        if args.start_in_game {
            app.add_systems(
                OnEnter(AppState::PlayerSelectionScreen),
                select_player_when_starting_in_game.run_if(run_once()),
            );
        }
    }
}
