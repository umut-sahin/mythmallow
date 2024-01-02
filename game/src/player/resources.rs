use crate::prelude::*;


/// Resource for the index of the selected player.
#[derive(Clone, Copy, Debug, Reflect, Resource)]
pub struct SelectedPlayerIndex {
    pub mythology_index: usize,
    pub player_index: usize,
}

/// Resource for the selected player.
#[derive(Clone, Debug, Deref, Resource)]
pub struct SelectedPlayer(pub Arc<dyn IPlayer>);
