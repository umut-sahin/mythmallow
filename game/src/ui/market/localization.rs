use crate::prelude::*;


/// Gets the localized text of the refresh button.
pub fn refresh_button(refresh_cost: Balance) -> LocalizedText {
    LocalizedText::Localized {
        key: "market-refresh-button",
        args: smallvec![("cost", format_smolstr!("{}", refresh_cost))],
        fallback: format!("Refresh - {}", refresh_cost).into(),
    }
}


/// Gets the localized text of lock buttons.
pub fn lock_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "market-lock-button",
        args: smallvec![],
        fallback: "Lock".into(),
    }
}

/// Gets the localized text of unlock buttons.
pub fn unlock_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "market-unlock-button",
        args: smallvec![],
        fallback: "Unlock".into(),
    }
}


/// Gets the localized text of the continue button.
pub fn continue_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "market-continue-button",
        args: smallvec![],
        fallback: "Continue".into(),
    }
}
