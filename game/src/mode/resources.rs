use crate::prelude::*;


/// Resource for the index of the selected game mode.
#[derive(Clone, Copy, Debug, Reflect, Resource)]
pub struct GameModeIndex(pub usize);


/// Resource for the selected game mode.
#[derive(Debug, Default, Reflect, Resource)]
pub struct GameMode<M: Mode>(pub M);
