use crate::prelude::*;


/// Gets the localized text of the won title.
pub fn won_title() -> LocalizedText {
    LocalizedText::Localized {
        key: "game-over-menu-won-title",
        args: smallvec![],
        fallback: "You won!".into(),
    }
}

/// Gets the localized text of the lost title.
pub fn lost_title() -> LocalizedText {
    LocalizedText::Localized {
        key: "game-over-menu-lost-title",
        args: smallvec![],
        fallback: "You lost!".into(),
    }
}


/// Gets the localized text of the play again button.
pub fn play_again_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "game-over-menu-play-again-button",
        args: smallvec![],
        fallback: "Play again".into(),
    }
}

/// Gets the localized text of the retry button.
pub fn retry_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "game-over-menu-retry-button",
        args: smallvec![],
        fallback: "Retry".into(),
    }
}


/// Gets the localized text of the return to main menu button.
pub fn return_to_main_menu_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "game-over-menu-return-to-main-menu-button",
        args: smallvec![],
        fallback: "Return to main menu".into(),
    }
}

/// Gets the localized text of the quit button.
pub fn quit_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "game-over-menu-quit-button",
        args: smallvec![],
        fallback: "Quit the game".into(),
    }
}
