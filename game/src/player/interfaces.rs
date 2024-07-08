use crate::{
    player::constants::*,
    prelude::*,
};


/// Interface for mythologies.
pub trait IMythology: Any + Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the mythology.
    fn id(&self) -> SmolStr;
    /// Gets the localized name of the mythology.
    fn name(&self) -> LocalizedText;
}


/// Interface for players.
pub trait IPlayer: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the player.
    fn id(&self) -> SmolStr;
    /// Gets the localized name of the player.
    fn name(&self) -> LocalizedText;

    /// Gets the base health of the player.
    fn health(&self) -> Health {
        Health(BASE_HEALTH)
    }
    /// Gets the base pickup range of the player.
    fn pickup_range(&self) -> PickupRange {
        PickupRange(BASE_PICKUP_RANGE)
    }
    /// Gets the base speed of the player.
    fn speed(&self) -> Speed {
        Speed(BASE_SPEED)
    }
    /// Gets the base speed multiplier of the player.
    fn speed_multiplier(&self) -> SpeedMultiplier {
        SpeedMultiplier::default()
    }
    /// Gets the base dodge change of the player.
    fn dodge_chance(&self) -> DodgeChance {
        DodgeChance::default()
    }
    /// Gets the base health regeneration of the player.
    fn hp_regeneration(&self) -> HpRegeneration {
        HpRegeneration::default()
    }


    /// Gets the collider of the player.
    fn collider(&self) -> Collider;
    /// Spawns the player.
    fn spawn(&self, world: &mut World);
}
