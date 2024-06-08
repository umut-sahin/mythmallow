use crate::{
    prelude::*,
    ui::settings_menu::systems::*,
};

/// Plugin for managing the settings menu.
#[derive(Default)]
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<SettingsMenu>();
        app.register_type::<SettingsMenuLanguageSettingContainer>();
        app.register_type::<SettingsMenuLanguageSettingPreviousButton>();
        app.register_type::<SettingsMenuLanguageSettingName>();
        app.register_type::<SettingsMenuLanguageSettingValue>();
        app.register_type::<SettingsMenuLanguageSettingNextButton>();
        app.register_type::<SettingsMenuBackButton>();

        // Setup localization.
        app.world.resource_mut::<LocaleAssets>().push("ui/settings_menu.ftl");

        // Add systems.
        app.add_systems(OnEnter(AppState::SettingsMenu), spawn_settings_menu);
        app.add_systems(OnEnter(GameState::Settings), spawn_settings_menu);
        app.add_systems(PreUpdate, navigation.in_set(SettingsMenuSystems));
        app.add_systems(
            PostUpdate,
            (
                back_button_interaction,
                language_setting_previous_button_interaction,
                language_setting_next_button_interaction,
            )
                .in_set(SettingsMenuSystems),
        );
        app.add_systems(OnExit(GameState::Settings), despawn_settings_menu);
        app.add_systems(OnExit(AppState::SettingsMenu), despawn_settings_menu);
    }
}
