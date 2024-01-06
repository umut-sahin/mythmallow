use crate::prelude::*;


/// Resource for the index of the selected game mode.
#[derive(Clone, Copy, Debug, Deref, Reflect, Resource)]
pub struct SelectedGameModeIndex(pub usize);


/// Resource for the current game mode.
#[derive(Debug, Default, Deref, Reflect, Resource)]
pub struct GameMode<M: IGameMode>(pub M);
