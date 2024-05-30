use crate::prelude::*;

/// Plugin for managing basic perks.
pub struct BasicPerksPlugin;

impl Plugin for BasicPerksPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Speedy>();

        // Get perk registry.
        let mut perk_registry = app.world.resource_mut::<PerkRegistry>();

        // Register perks.
        for rarity in Rarity::iter() {
            perk_registry.register(Healthy { rarity });
            perk_registry.register(Speedy { rarity });
        }
    }
}
