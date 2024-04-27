use crate::{
    player::constants::*,
    prelude::*,
};


/// Interface for mythologies.
pub trait IMythology: Any + Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the mythology.
    fn id(&self) -> SmolStr;
    /// Gets the name of the mythology.
    fn name(&self) -> SmolStr;
}


/// Interface for players.
pub trait IPlayer: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the player.
    fn id(&self) -> SmolStr;
    /// Gets the name of the player.
    fn name(&self) -> SmolStr;

    /// Gets the health of the player.
    fn health(&self) -> Health {
        Health(BASE_HEALTH)
    }
    /// Gets the pickup range of the player.
    fn pickup_range(&self) -> PickupRange {
        PickupRange(BASE_PICKUP_RANGE)
    }
    /// Gets the speed of the player.
    fn speed(&self) -> Speed {
        Speed(BASE_SPEED)
    }

    /// Gets the collider of the player.
    fn collider(&self) -> Collider;
    /// Spawns the player.
    fn spawn(&self, world: &mut World);
}
