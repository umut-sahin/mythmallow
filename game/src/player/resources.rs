use crate::prelude::*;


/// Resource for the index of the selected mythology.
#[derive(Clone, Copy, Debug, Deref, Reflect, Resource)]
pub struct SelectedMythologyIndex(pub usize);


/// Resource for the index of the selected player.
#[derive(Clone, Copy, Debug, Deref, Reflect, Resource)]
pub struct SelectedPlayerIndex(pub usize);
