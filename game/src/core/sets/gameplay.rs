use crate::prelude::*;

/// Systems to run in the game.
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum GameplaySystems {
    Camera,
    Combat,
    Enemy,
    GameMode,
    Input,
    Movement,
    Physics,
    Player,
}

impl GameplaySystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(
            app_state: Res<State<AppState>>,
            game_state: Res<State<GameState>>,
        ) -> bool {
            *app_state == AppState::Game && *game_state == GameState::Playing
        }

        for set in GameplaySystems::iter() {
            app.configure_sets(FixedUpdate, set.run_if(run_condition));
            app.configure_sets(PreUpdate, set.run_if(run_condition));
            app.configure_sets(Update, set.run_if(run_condition));
            app.configure_sets(PostUpdate, set.run_if(run_condition));
        }
    }
}
