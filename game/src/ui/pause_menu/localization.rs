use crate::prelude::*;


/// Gets the localized text of the resume button.
pub fn resume_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "pause-menu-resume-button",
        args: smallvec![],
        fallback: "Resume".into(),
    }
}

/// Gets the localized text of the settings button.
pub fn settings_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "pause-menu-settings-button",
        args: smallvec![],
        fallback: "Settings".into(),
    }
}

/// Gets the localized text of the return to main menu button.
pub fn return_to_main_menu_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "pause-menu-return-to-main-menu-button",
        args: smallvec![],
        fallback: "Return to main menu".into(),
    }
}

/// Gets the localized text of the quit button.
pub fn quit_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "pause-menu-quit-button",
        args: smallvec![],
        fallback: "Quit to desktop".into(),
    }
}
