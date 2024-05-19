use crate::{
    prelude::*,
    ui::{
        diagnostics_overlay::plugin::DiagnosticsOverlayPlugin,
        enemy_selection_screen::plugin::EnemySelectionScreenPlugin,
        game_mode_selection_screen::plugin::GameModeSelectionScreenPlugin,
        game_over_menu::plugin::GameOverMenuPlugin,
        hud::plugin::HudPlugin,
        main_menu::plugin::MainMenuPlugin,
        market::plugin::MarketPlugin,
        pause_menu::plugin::PauseMenuPlugin,
        player_selection_screen::plugin::PlayerSelectionScreenPlugin,
        widget::plugin::WidgetPlugin,
    },
};

/// Plugin for managing the user interface.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Add sub-plugins.
        app.add_plugins(WidgetPlugin);
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(GameModeSelectionScreenPlugin);
        app.add_plugins(PlayerSelectionScreenPlugin);
        app.add_plugins(EnemySelectionScreenPlugin);
        app.add_plugins(MarketPlugin);
        app.add_plugins(PauseMenuPlugin);
        app.add_plugins(GameOverMenuPlugin);
        app.add_plugins(HudPlugin);
        app.add_plugins(DiagnosticsOverlayPlugin);
    }
}
