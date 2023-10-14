use crate::prelude::*;


/// State of the application.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}


/// State of the game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}
