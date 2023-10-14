use crate::{
    core::systems::*,
    prelude::*,
};

/// Plugin for the core logic of the application.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AppState>();
        app.register_type::<GameState>();

        app.add_state::<AppState>();
        app.add_state::<GameState>();

        MainMenuSystems::configure(app);
        GamePlaySystems::configure(app);
        PauseMenuSystems::configure(app);
        GameOverMenuSystems::configure(app);
        MenuSystems::configure(app);

        app.add_systems(OnEnter(AppState::Restart), restart);
    }
}
