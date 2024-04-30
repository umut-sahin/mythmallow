use crate::prelude::*;


/// Resource for the index of the selected mythology.
#[derive(Clone, Copy, Debug, Deref, Reflect, Resource)]
pub struct SelectedMythologyIndex(pub usize);


/// Resource for the index of the selected player.
#[derive(Clone, Copy, Debug, Deref, Reflect, Resource)]
pub struct SelectedPlayerIndex(pub usize);


/// Resource for god mode.
#[derive(Clone, Copy, Debug, Reflect, Resource)]
pub struct GodMode {
    pub is_enabled: bool,
}

impl Default for GodMode {
    fn default() -> GodMode {
        GodMode { is_enabled: false }
    }
}
