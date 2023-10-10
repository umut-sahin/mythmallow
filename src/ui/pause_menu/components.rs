use crate::prelude::*;


/// Tag component for the pause menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct PauseMenu;


/// Tag component for resume button in the pause menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct PauseMenuResumeButton;


/// Tag component for return to main menu button in the pause menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct PauseMenuReturnToMainMenuButton;


/// Tag component for quit to desktop button in the pause menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct PauseMenuQuitToDesktopButton;
