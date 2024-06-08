use crate::prelude::*;


/// Gets the localized text of the experience bar.
pub fn experience_bar(level: &Level) -> LocalizedText {
    LocalizedText::Localized {
        key: "hud-experience-bar",
        args: smallvec![("level", format_smolstr!("{}", level.0))],
        fallback: format!("Level {}", level.0).into(),
    }
}
