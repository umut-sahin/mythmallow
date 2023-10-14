use crate::{
    combat::systems::*,
    prelude::*,
    status_effect::systems::cooldown,
};

/// Plugin for managing the combat.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RemainingHealth>();
        app.register_type::<Cooldown<Attack>>();

        app.add_systems(PreUpdate, cooldown::<Attack>.in_set(GamePlaySystems::Combat));
        app.add_systems(
            Update,
            (damage_player_on_contact_with_enemies, player_death)
                .chain()
                .in_set(GamePlaySystems::Combat),
        );
    }
}
