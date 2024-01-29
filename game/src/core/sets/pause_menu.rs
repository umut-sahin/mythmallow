use crate::prelude::*;

/// Systems to run in the pause menu.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct PauseMenuSystems;

impl PauseMenuSystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(
            app_state: Res<State<AppState>>,
            game_state: Res<State<GameState>>,
            console_state: Res<ConsoleState>,
        ) -> bool {
            *app_state == AppState::Game && *game_state == GameState::Paused && !console_state.open
        }

        app.configure_sets(PreUpdate, Self.run_if(run_condition));
        app.configure_sets(Update, Self.run_if(run_condition));
        app.configure_sets(PostUpdate, Self.run_if(run_condition));
    }
}
