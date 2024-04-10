use crate::{
    input::systems::*,
    prelude::*,
};

/// Plugin for managing user inputs.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        // Setup actions.
        GlobalAction::setup(app);
        MainMenuAction::setup(app);
        PlayerSelectionScreenAction::setup(app);
        GameAction::setup(app);
        PauseMenuAction::setup(app);
        GameOverMenuAction::setup(app);

        // Add systems.
        {
            app.add_systems(Update, pause_on_losing_focus.in_set(GameplaySystems::Input));

            app.add_systems(Update, toggle_fullscreen);
            app.add_systems(Update, toggle_diagnostics_overlay);

            #[cfg(feature = "development")]
            app.add_systems(Update, toggle_physics_gizmos);
        }
    }
}
