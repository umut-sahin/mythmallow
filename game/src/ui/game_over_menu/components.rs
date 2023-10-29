use crate::prelude::*;


/// Tag component for the game over menu.
#[derive(Component, Debug, Reflect)]
pub struct GameOverMenu;


/// Tag component for the title in the game over menu.
#[derive(Component, Debug, Reflect)]
pub struct GameOverMenuTitle;


/// Tag component for the play again button in the game over menu.
#[derive(Component, Debug, Reflect)]
pub struct GameOverMenuPlayAgainButton;


/// Tag component for the retry button in the game over menu.
#[derive(Component, Debug, Reflect)]
pub struct GameOverMenuRetryButton;


/// Tag component for the return to main menu button in the game over menu.
#[derive(Component, Debug, Reflect)]
pub struct GameOverMenuReturnToMainMenuButton;


/// Tag component for the quit button in the game over menu.
#[derive(Component, Debug, Reflect)]
pub struct GameOverMenuQuitButton;
