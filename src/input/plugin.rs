use crate::{
    input::systems::*,
    prelude::*,
};

/// Plugin for managing user inputs.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        GlobalAction::setup(app);
        MainMenuAction::setup(app);
        GameAction::setup(app);
        PauseMenuAction::setup(app);
        GameOverMenuAction::setup(app);

        app.add_systems(Update, toggle_fullscreen);

        let general_settings = app.world.resource::<Persistent<GeneralSettings>>();
        if general_settings.pause_on_losing_focus {
            app.add_systems(Update, pause_on_losing_focus.in_set(GameplaySystems::Input));
        }
    }
}
