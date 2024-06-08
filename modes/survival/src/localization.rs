use crate::prelude::*;


/// Gets the localized text of the current wave.
pub fn current_wave(current_wave: &CurrentWave) -> LocalizedText {
    LocalizedText::Localized {
        key: "survival-mode-hud-current-wave",
        args: smallvec![("wave", format_smolstr!("{}", current_wave.0))],
        fallback: format!("Wave {}", current_wave.0).into(),
    }
}
