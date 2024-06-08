use crate::prelude::*;


/// Resource for the previously selected widget in the pause menu.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
pub struct PreviouslySelectedPauseMenuWidget(pub Entity);
