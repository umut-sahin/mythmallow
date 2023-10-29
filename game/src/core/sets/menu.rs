use crate::prelude::*;

/// Systems to run in menus.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct MenuSystems;

impl MenuSystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(
            app_state: Res<State<AppState>>,
            game_state: Res<State<GameState>>,
        ) -> bool {
            match app_state.get() {
                AppState::MainMenu => true,
                AppState::PlayerSelectionScreen => true,
                AppState::GameModeSelectionScreen => true,
                AppState::Game => {
                    match game_state.get() {
                        GameState::None => false,
                        GameState::Initialization => false,
                        GameState::Loading => false,
                        GameState::Playing => false,
                        GameState::Paused => true,
                        GameState::Won => false,
                        GameState::Over => true,
                        GameState::Restart => false,
                    }
                },
            }
        }

        app.configure_sets(PreUpdate, Self.run_if(run_condition));
        app.configure_sets(Update, Self.run_if(run_condition));
        app.configure_sets(PostUpdate, Self.run_if(run_condition));
    }
}
