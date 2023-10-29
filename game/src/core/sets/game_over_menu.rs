use crate::prelude::*;

/// Systems to run in the game over menu.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct GameOverMenuSystems;

impl GameOverMenuSystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(
            app_state: Res<State<AppState>>,
            game_state: Res<State<GameState>>,
        ) -> bool {
            *app_state == AppState::Game && *game_state == GameState::Over
        }

        app.configure_sets(PreUpdate, Self.run_if(run_condition));
        app.configure_sets(Update, Self.run_if(run_condition));
        app.configure_sets(PostUpdate, Self.run_if(run_condition));
    }
}
