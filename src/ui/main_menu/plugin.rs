use crate::{
    prelude::*,
    ui::main_menu::systems::*,
};

/// Plugin for managing the main menu.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainMenu>();
        app.register_type::<MainMenuPlayButton>();
        app.register_type::<MainMenuQuitButton>();

        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        app.add_systems(Update, navigation.in_set(MainMenuSystems));
        app.add_systems(
            PostUpdate,
            (start_game_on_play_button_click, quit_on_quit_button_click).in_set(MainMenuSystems),
        );
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
