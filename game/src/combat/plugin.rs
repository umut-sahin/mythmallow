use crate::{
    combat::systems::*,
    prelude::*,
};

/// Plugin for managing the combat.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<RemainingHealth>();
        app.register_type::<Cooldown<Attack>>();

        // Add systems.
        app.add_systems(PreUpdate, cooldown::<Attack>.in_set(GameplaySystems::Combat));
        app.add_systems(
            Update,
            (damage_player_on_contact_with_enemies, player_death)
                .chain()
                .in_set(GameplaySystems::Combat),
        );
    }
}
