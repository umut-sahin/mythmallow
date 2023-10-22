use crate::prelude::*;


/// State of the application.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum AppState {
    #[default]
    MainMenu,
    GameModeSelectionScreen,
    Game,
}


/// State of the game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum GameState {
    #[default]
    None,
    Initialization,
    Loading,
    Playing,
    Paused,
    Won,
    Over,
    Restart,
}


/// State of the diagnostics overlay.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum DiagnosticsOverlayState {
    #[default]
    Disabled,
    Enabled,
}
