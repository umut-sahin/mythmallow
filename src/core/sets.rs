use crate::prelude::*;


/// Systems to run in the main menu.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct MainMenuSystems;

impl MainMenuSystems {
    /// Configure run conditions for the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(app_state: Res<State<AppState>>) -> bool {
            match app_state.get() {
                AppState::MainMenu => true,
                AppState::Game => false,
                AppState::Restart => false,
            }
        }

        app.configure_set(PreUpdate, MainMenuSystems.run_if(run_condition));
        app.configure_set(Update, MainMenuSystems.run_if(run_condition));
        app.configure_set(PostUpdate, MainMenuSystems.run_if(run_condition));
    }
}


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


/// Systems to run in the pause menu.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct PauseMenuSystems;

impl PauseMenuSystems {
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
                        GameState::Playing => false,
                        GameState::Paused => true,
                        GameState::Won => false,
                        GameState::Lost => false,
                    }
                },
                AppState::Restart => false,
            }
        }

        app.configure_set(PreUpdate, PauseMenuSystems.run_if(run_condition));
        app.configure_set(Update, PauseMenuSystems.run_if(run_condition));
        app.configure_set(PostUpdate, PauseMenuSystems.run_if(run_condition));
    }
}


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
                        GameState::Playing => false,
                        GameState::Paused => false,
                        GameState::Won => true,
                        GameState::Lost => true,
                    }
                },
                AppState::Restart => false,
            }
        }

        app.configure_set(PreUpdate, PauseMenuSystems.run_if(run_condition));
        app.configure_set(Update, PauseMenuSystems.run_if(run_condition));
        app.configure_set(PostUpdate, PauseMenuSystems.run_if(run_condition));
    }
}


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
