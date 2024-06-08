use crate::prelude::*;


/// Gets the localized text of the previous language setting.
pub fn language_setting_previous_button() -> LocalizedText {
    LocalizedText::Constant { text: "<".into() }
}

/// Gets the localized text of the language setting name.
pub fn language_setting_name() -> LocalizedText {
    LocalizedText::Localized {
        key: "settings-menu-language-setting-name",
        args: smallvec![],
        fallback: "Language:".into(),
    }
}

/// Gets the localized text of the language setting value.
pub fn language_setting_value() -> LocalizedText {
    LocalizedText::Localized {
        key: "settings-menu-language-setting-value",
        args: smallvec![],
        fallback: "English".into(),
    }
}

/// Gets the localized text of the next language setting.
pub fn language_setting_next_button() -> LocalizedText {
    LocalizedText::Constant { text: ">".into() }
}


/// Gets the localized text of the back button.
pub fn back_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "settings-menu-back-button",
        args: smallvec![],
        fallback: "Back".into(),
    }
}
