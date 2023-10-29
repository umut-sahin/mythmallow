mod game;
mod game_over_menu;
mod global;
mod main_menu;
mod pause_menu;
mod player_selection_screen;

pub use {
    game::GameAction,
    game_over_menu::GameOverMenuAction,
    global::GlobalAction,
    main_menu::MainMenuAction,
    pause_menu::PauseMenuAction,
    player_selection_screen::PlayerSelectionScreenAction,
};
