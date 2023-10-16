use crate::prelude::*;

/// Systems to run in the game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum GamePlaySystems {
    Camera,
    Combat,
    Enemy,
    Input,
    Map,
    Movement,
    Physics,
    Player,
}

impl GamePlaySystems {
    /// Configure run conditions for the system set.
    pub fn configure(app: &mut App) {
        use GamePlaySystems::*;
        {
            fn run_condition(
                app_state: Res<State<AppState>>,
                game_state: Res<State<GameState>>,
            ) -> bool {
                match app_state.get() {
                    AppState::MainMenu => false,
                    AppState::Game => {
                        match game_state.get() {
                            GameState::Playing => true,
                            GameState::Paused => false,
                            GameState::Won => false,
                            GameState::Lost => false,
                        }
                    },
                    AppState::Restart => false,
                }
            }

            let all_sets = [Camera, Combat, Enemy, Input, Map, Movement, Physics, Player];
            for set in all_sets {
                app.configure_set(FixedUpdate, set.run_if(run_condition));
                app.configure_set(PreUpdate, set.run_if(run_condition));
                app.configure_set(Update, set.run_if(run_condition));
                app.configure_set(PostUpdate, set.run_if(run_condition));
            }
        }
        {
            let after_physics_set = [Camera, Combat, Enemy, Input, Map, Movement, Player];
            for set in after_physics_set {
                app.configure_set(FixedUpdate, Physics.before(set));
                app.configure_set(PreUpdate, Physics.before(set));
                app.configure_set(Update, Physics.before(set));
                app.configure_set(PostUpdate, Physics.before(set));
            }
        }
    }
}
