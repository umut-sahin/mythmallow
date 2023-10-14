use crate::prelude::*;


/// Tag component for the game over menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct GameOverMenu;


/// Tag component for the title in the game over menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct GameOverMenuTitle;


/// Tag component for play again button in the game over menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct GameOverMenuPlayAgainButton;


/// Tag component for retry button in the game over menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct GameOverMenuRetryButton;


/// Tag component for return to main menu button in the game over menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct GameOverMenuReturnToMainMenuButton;


/// Tag component for quit to desktop button in the game over menu.
#[derive(Component, Debug, Default, Reflect)]
pub struct GameOverMenuQuitToDesktopButton;
