use crate::prelude::*;

/// Systems to run in the game.
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum GameplaySystems {
    Camera,
    Combat,
    Enemy,
    GameMode,
    Input,
    Map,
    Movement,
    Physics,
    Player,
}

impl GameplaySystems {
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
                        GameState::Playing => true,
                        GameState::Paused => false,
                        GameState::Won => false,
                        GameState::Over => false,
                        GameState::Restart => false,
                    }
                },
            }
        }

        for set in GameplaySystems::iter() {
            app.configure_set(FixedUpdate, set.run_if(run_condition));
            app.configure_set(PreUpdate, set.run_if(run_condition));
            app.configure_set(Update, set.run_if(run_condition));
            app.configure_set(PostUpdate, set.run_if(run_condition));
        }

        {
            let physics = GameplaySystems::Physics;

            let mut after_physics_set = GameplaySystems::iter().collect::<HashSet<_>>();
            after_physics_set.remove(&physics);

            for set in after_physics_set {
                app.configure_set(FixedUpdate, physics.before(set));
                app.configure_set(PreUpdate, physics.before(set));
                app.configure_set(Update, physics.before(set));
                app.configure_set(PostUpdate, physics.before(set));
            }
        }
    }
}
