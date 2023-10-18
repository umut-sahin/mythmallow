use crate::prelude::*;

/// Systems to run in the main menu.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct MainMenuSystems;

impl MainMenuSystems {
    /// Configure run conditions for the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(app_state: Res<State<AppState>>) -> bool {
            match app_state.get() {
                AppState::MainMenu => true,
                AppState::GameModeSelectionScreen => false,
                AppState::Game => false,
            }
        }

        app.configure_set(PreUpdate, MainMenuSystems.run_if(run_condition));
        app.configure_set(Update, MainMenuSystems.run_if(run_condition));
        app.configure_set(PostUpdate, MainMenuSystems.run_if(run_condition));
    }
}
