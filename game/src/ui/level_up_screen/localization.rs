use crate::prelude::*;


/// Gets the localized text of select buttons.
pub fn select_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "level-up-screen-select-button",
        args: smallvec![],
        fallback: "Select".into(),
    }
}


/// Gets the localized text of the reroll button.
pub fn reroll_button(reroll_cost: Balance) -> LocalizedText {
    LocalizedText::Localized {
        key: "level-up-screen-reroll-button",
        args: smallvec![("cost", format_smolstr!("{}", reroll_cost))],
        fallback: format!("Reroll - {}", reroll_cost).into(),
    }
}
