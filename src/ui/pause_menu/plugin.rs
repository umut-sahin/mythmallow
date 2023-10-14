use crate::{
    prelude::*,
    ui::pause_menu::systems::*,
};

/// Plugin for managing the pause menu.
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PauseMenu>();
        app.register_type::<PauseMenuResumeButton>();
        app.register_type::<PauseMenuReturnToMainMenuButton>();
        app.register_type::<PauseMenuQuitToDesktopButton>();

        app.add_systems(OnEnter(GameState::Paused), spawn_pause_menu);
        app.add_systems(Update, navigation.in_set(PauseMenuSystems));
        app.add_systems(
            PostUpdate,
            (
                resume_button_interaction,
                return_to_main_menu_button_interaction,
                quit_to_desktop_button_interaction,
            )
                .in_set(PauseMenuSystems),
        );
        app.add_systems(OnExit(GameState::Paused), despawn_pause_menu);
    }
}
