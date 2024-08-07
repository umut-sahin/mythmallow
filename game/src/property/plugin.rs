use crate::{
    prelude::*,
    property::systems::*,
};

/// Plugin for managing the properties of game objects.
pub struct PropertyPlugin;

impl Plugin for PropertyPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Damage>();
        app.register_type::<DodgeChance>();
        app.register_type::<Health>();
        app.register_type::<PickupRange>();
        app.register_type::<Range>();
        app.register_type::<Speed>();
        app.register_type::<SpeedMultiplier>();
        app.register_type::<HpRegeneration>();

        // Add systems.
        app.add_systems(PreUpdate, hp_regeneration.in_set(GameplaySystems::Property));
    }
}
