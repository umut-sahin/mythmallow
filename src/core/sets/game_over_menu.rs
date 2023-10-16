use crate::prelude::*;

/// Systems to run in the game over menu.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct GameOverMenuSystems;

impl GameOverMenuSystems {
    /// Configure run conditions for the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(
            app_state: Res<State<AppState>>,
            game_state: Res<State<GameState>>,
        ) -> bool {
            match app_state.get() {
                AppState::MainMenu => false,
                AppState::Game => {
                    match game_state.get() {
                        GameState::None => false,
                        GameState::Setup => false,
                        GameState::Loading => false,
                        GameState::Playing => false,
                        GameState::Paused => false,
                        GameState::Won => false,
                        GameState::Over => true,
                    }
                },
                AppState::Restart => false,
            }
        }

        app.configure_set(PreUpdate, GameOverMenuSystems.run_if(run_condition));
        app.configure_set(Update, GameOverMenuSystems.run_if(run_condition));
        app.configure_set(PostUpdate, GameOverMenuSystems.run_if(run_condition));
    }
}
