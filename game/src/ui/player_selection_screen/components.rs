use crate::prelude::*;


/// Tag component for the player selection screen.
#[derive(Component, Debug, Reflect)]
pub struct PlayerSelectionScreen;


/// Tag component for player buttons in the player selection screen.
#[derive(Component, Debug, Reflect)]
pub struct PlayerSelectionScreenPlayerButton {
    // Index of the mythology the button represents.
    pub mythology_index: SelectedMythologyIndex,
    // Index of the player the button represents.
    pub player_index: SelectedPlayerIndex,
}


/// Tag component for the back button in the player selection screen.
#[derive(Component, Debug, Reflect)]
pub struct PlayerSelectionScreenBackButton;
