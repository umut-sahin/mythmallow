use crate::prelude::*;


/// Tag component for the game mode selection screen.
#[derive(Component, Debug, Reflect)]
pub struct GameModeSelectionScreen;


/// Tag component for the game mode buttons in the game mode selection screen.
#[derive(Component, Debug, Reflect)]
pub struct GameModeSelectionScreenGameModeButton {
    // Index of the game mode the button represents.
    pub game_mode_index: SelectedGameModeIndex,
}


/// Tag component for the back button in the game mode selection screen.
#[derive(Component, Debug, Reflect)]
pub struct GameModeSelectionScreenBackButton;
