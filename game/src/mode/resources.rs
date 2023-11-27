use crate::prelude::*;


/// Resource for the index of the selected game mode.
#[derive(Clone, Copy, Debug, Deref, Reflect, Resource)]
pub struct SelectedGameModeIndex(pub usize);


/// Resource for the selected game mode.
#[derive(Clone, Debug, Deref, Resource)]
pub struct SelectedGameMode(pub Arc<dyn Mode>);


/// Resource for the current game mode.
#[derive(Debug, Default, Deref, Reflect, Resource)]
pub struct GameMode<M: Mode>(pub M);
