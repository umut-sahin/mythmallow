use crate::prelude::*;


/// Interface for game modes.
pub trait Mode: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the game mode.
    fn id(&self) -> SmolStr;
    /// Gets the name of the game mode.
    fn name(&self) -> SmolStr;

    /// Initializes the game mode.
    fn initialize(&self, world: &mut World);
    /// Deinitializes the game mode.
    fn deinitialize(&self, world: &mut World);
}
