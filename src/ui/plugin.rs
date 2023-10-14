use crate::{
    prelude::*,
    ui::{
        game_over_menu::plugin::GameOverMenuPlugin,
        main_menu::plugin::MainMenuPlugin,
        pause_menu::plugin::PauseMenuPlugin,
        widget::plugin::WidgetPlugin,
    },
};

/// Plugin for managing the user interface.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WidgetPlugin);
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(PauseMenuPlugin);
        app.add_plugins(GameOverMenuPlugin);
    }
}
