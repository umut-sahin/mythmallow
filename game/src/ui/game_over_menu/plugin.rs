use crate::{
    prelude::*,
    ui::game_over_menu::systems::*,
};

/// Plugin for managing the game over menu.
pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<GameOverMenu>();
        app.register_type::<GameOverMenuTitle>();
        app.register_type::<GameOverMenuPlayAgainButton>();
        app.register_type::<GameOverMenuRetryButton>();
        app.register_type::<GameOverMenuReturnToMainMenuButton>();
        app.register_type::<GameOverMenuQuitButton>();

        // Add systems.
        app.add_systems(OnEnter(GameState::Over), spawn_game_over_menu);
        app.add_systems(Update, navigation.in_set(GameOverMenuSystems));
        app.add_systems(
            PostUpdate,
            (
                play_again_button_interaction,
                retry_button_interaction,
                return_to_main_menu_button_interaction,
                quit_button_interaction,
            )
                .in_set(GameOverMenuSystems),
        );
        app.add_systems(OnExit(GameState::Over), despawn_game_over_menu);
    }
}
