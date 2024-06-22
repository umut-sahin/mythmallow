use crate::prelude::*;

/// Plugin for managing basic perks.
pub struct BasicPerksPlugin;

impl Plugin for BasicPerksPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Healthy>();
        app.register_type::<Speedy>();
        app.register_type::<Dodgy>();

        // Setup localization.
        app.world.resource_mut::<LocaleAssets>().push("content/perks/basic.ftl");

        // Get perk registry.
        let mut perk_registry = app.world.resource_mut::<PerkRegistry>();

        // Register perks.
        for rarity in Rarity::iter() {
            perk_registry.register(Healthy { rarity });
            perk_registry.register(Speedy { rarity });
            perk_registry.register(Dodgy { rarity });
        }
    }
}
