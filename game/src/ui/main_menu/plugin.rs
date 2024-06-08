use crate::{
    prelude::*,
    ui::main_menu::systems::*,
};

/// Plugin for managing the main menu.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<MainMenu>();
        app.register_type::<MainMenuPlayButton>();
        app.register_type::<MainMenuSettingsButton>();
        app.register_type::<MainMenuQuitButton>();

        // Setup localization.
        app.world.resource_mut::<LocaleAssets>().push("ui/main_menu.ftl");

        // Add systems.
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        app.add_systems(Update, navigation.in_set(MainMenuSystems));
        app.add_systems(
            PostUpdate,
            (play_button_interaction, settings_button_interaction, quit_button_interaction)
                .in_set(MainMenuSystems),
        );
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
