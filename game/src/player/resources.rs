use crate::prelude::*;


/// Resource for the index of the selected player.
#[derive(Clone, Copy, Debug, Reflect, Resource)]
pub struct PlayerIndex {
    pub mythology_index: usize,
    pub player_index: usize,
}
