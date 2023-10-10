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

        app.add_systems(Update, toggle_fullscreen);
    }
}
