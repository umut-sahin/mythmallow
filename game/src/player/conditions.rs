use crate::prelude::*;


/// Condition to run when god mode is enabled.
pub fn god_mode_is_enabled(god_mode: Res<GodMode>) -> bool {
    god_mode.is_enabled
}

/// Condition to run when god mode is disabled.
pub fn god_mode_is_disabled(god_mode: Res<GodMode>) -> bool {
    !god_mode.is_enabled
}
