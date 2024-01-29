use crate::prelude::*;

/// Systems to run in the game mode selection screen.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct GameModeSelectionScreenSystems;

impl GameModeSelectionScreenSystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(
            app_state: Res<State<AppState>>,
            console_state: Res<ConsoleState>,
        ) -> bool {
            *app_state == AppState::GameModeSelectionScreen && !console_state.open
        }

        app.configure_sets(PreUpdate, Self.run_if(run_condition));
        app.configure_sets(Update, Self.run_if(run_condition));
        app.configure_sets(PostUpdate, Self.run_if(run_condition));
    }
}
