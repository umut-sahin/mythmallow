use crate::prelude::*;


/// State of the application.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum AppState {
    #[default]
    LoadingInitialLocalization,
    MainMenu,
    SettingsMenu,
    GameModeSelectionScreen,
    PlayerSelectionScreen,
    EnemySelectionScreen,
    Game,
}


/// State of the game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum GameState {
    #[default]
    Transition,
    Initialization,
    Loading,
    Playing,
    LevelUpScreen,
    Market,
    Paused,
    Settings,
    Restart,
    Won,
    Over,
}

impl GameState {
    /// Gets whether the physics is enabled in the state.
    pub fn physics_enabled(&self) -> bool {
        *self == GameState::Playing
    }
}


/// State of the localization.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum LocalizationState {
    #[default]
    Loading,
    Ready,
}


/// State of the diagnostics overlay.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum DiagnosticsOverlayState {
    #[default]
    Disabled,
    Enabled,
}
