use crate::prelude::*;


/// Interface for mythologies.
pub trait Mythology: Any + Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the mythology.
    fn id(&self) -> SmolStr;
    /// Gets the name of the mythology.
    fn name(&self) -> SmolStr;
}


/// Interface for players.
pub trait Playable: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the player.
    fn id(&self) -> SmolStr;
    /// Gets the name of the player.
    fn name(&self) -> SmolStr;

    /// Spawns the player.
    fn spawn(&self, world: &mut World);
}
