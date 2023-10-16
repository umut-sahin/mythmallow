use crate::prelude::*;

/// Systems to run in the menus.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct MenuSystems;

impl MenuSystems {
    /// Configure run conditions for the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(
            app_state: Res<State<AppState>>,
            game_state: Res<State<GameState>>,
        ) -> bool {
            match app_state.get() {
                AppState::MainMenu => true,
                AppState::Game => {
                    match game_state.get() {
                        GameState::Playing => false,
                        GameState::Paused => true,
                        GameState::Won => true,
                        GameState::Lost => true,
                    }
                },
                AppState::Restart => false,
            }
        }

        app.configure_set(PreUpdate, MenuSystems.run_if(run_condition));
        app.configure_set(Update, MenuSystems.run_if(run_condition));
        app.configure_set(PostUpdate, MenuSystems.run_if(run_condition));
    }
}
