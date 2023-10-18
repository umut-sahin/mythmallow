use crate::prelude::*;

/// Systems to run in the pause menu.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct PauseMenuSystems;

impl PauseMenuSystems {
    /// Configure run conditions for the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(
            app_state: Res<State<AppState>>,
            game_state: Res<State<GameState>>,
        ) -> bool {
            match app_state.get() {
                AppState::MainMenu => false,
                AppState::GameModeSelectionScreen => false,
                AppState::Game => {
                    match game_state.get() {
                        GameState::None => false,
                        GameState::Setup => false,
                        GameState::Loading => false,
                        GameState::Playing => false,
                        GameState::Paused => true,
                        GameState::Won => false,
                        GameState::Over => false,
                        GameState::Restart => false,
                    }
                },
            }
        }

        app.configure_set(PreUpdate, PauseMenuSystems.run_if(run_condition));
        app.configure_set(Update, PauseMenuSystems.run_if(run_condition));
        app.configure_set(PostUpdate, PauseMenuSystems.run_if(run_condition));
    }
}
