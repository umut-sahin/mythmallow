use crate::prelude::*;


/// Tag component for the pause menu.
#[derive(Component, Debug, Reflect)]
pub struct PauseMenu;


/// Tag component for the resume button in the pause menu.
#[derive(Component, Debug, Reflect)]
pub struct PauseMenuResumeButton;


/// Tag component for the return to main menu button in the pause menu.
#[derive(Component, Debug, Reflect)]
pub struct PauseMenuReturnToMainMenuButton;


/// Tag component for the quit button in the pause menu.
#[derive(Component, Debug, Reflect)]
pub struct PauseMenuQuitButton;
