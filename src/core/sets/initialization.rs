use crate::prelude::*;

/// Systems to run when initializing the game.
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum InitializationSystems {
    First,
    Player,
    Enemy,
    GameMode,
    Map,
    Last,
    Done,
}

impl InitializationSystems {
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
                        GameState::Initialization => true,
                        GameState::Loading => false,
                        GameState::Playing => false,
                        GameState::Paused => false,
                        GameState::Won => false,
                        GameState::Over => false,
                        GameState::Restart => false,
                    }
                },
            }
        }

        for stage in InitializationSystems::iter() {
            app.configure_set(PreUpdate, stage.run_if(run_condition));
            app.configure_set(Update, stage.run_if(run_condition));
            app.configure_set(PostUpdate, stage.run_if(run_condition));
        }

        for (current, next) in
            InitializationSystems::iter().zip(InitializationSystems::iter().skip(1))
        {
            app.configure_set(OnEnter(GameState::Initialization), current.before(next));
            app.configure_set(PreUpdate, current.before(next));
            app.configure_set(Update, current.before(next));
            app.configure_set(PostUpdate, current.before(next));
        }
    }
}
