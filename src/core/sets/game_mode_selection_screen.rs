use crate::prelude::*;


/// Systems to run in the game mode selection screen.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct GameModeSelectionScreenSystems;

impl GameModeSelectionScreenSystems {
    /// Configure run conditions for the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(app_state: Res<State<AppState>>) -> bool {
            match app_state.get() {
                AppState::MainMenu => false,
                AppState::GameModeSelectionScreen => true,
                AppState::Game => false,
            }
        }

        app.configure_set(PreUpdate, GameModeSelectionScreenSystems.run_if(run_condition));
        app.configure_set(Update, GameModeSelectionScreenSystems.run_if(run_condition));
        app.configure_set(PostUpdate, GameModeSelectionScreenSystems.run_if(run_condition));
    }
}
