use crate::prelude::*;


/// Gets the localized text of the back button.
pub fn back_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "game-mode-selection-screen-back-button",
        args: smallvec![],
        fallback: "Back".into(),
    }
}
