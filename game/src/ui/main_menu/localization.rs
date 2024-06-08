use crate::prelude::*;


/// Gets the localized text of the play button.
pub fn play_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "main-menu-play-button",
        args: smallvec![],
        fallback: "Play".into(),
    }
}

/// Gets the localized text of the settings button.
pub fn settings_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "main-menu-settings-button",
        args: smallvec![],
        fallback: "Settings".into(),
    }
}

/// Gets the localized text of the quit button.
pub fn quit_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "main-menu-quit-button",
        args: smallvec![],
        fallback: "Quit the game".into(),
    }
}
