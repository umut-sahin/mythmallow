use crate::{
    combat::systems::*,
    prelude::*,
};

/// Plugin for managing the combat.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<DamageCooldown>();
        app.register_type::<Cooldown<Attack>>();
        app.register_type::<OriginatorName>();
        app.register_type::<Projectile>();
        app.register_type::<RemainingHealth>();

        // Add systems.
        app.add_systems(PreUpdate, cooldown::<Attack>.in_set(GameplaySystems::Combat));
        app.add_systems(Update, (damage_player, damage_enemies).in_set(GameplaySystems::Combat));
        app.add_systems(
            PostUpdate,
            (
                player_death.run_if(god_mode_is_disabled),
                enemy_death,
                despawn_projectiles_on_contact,
            )
                .in_set(GameplaySystems::Combat),
        );
        app.add_systems(OnEnter(GameState::Won), despawn_projectiles);
        app.add_systems(OnEnter(GameState::Over), despawn_projectiles);
        app.add_systems(
            OnEnter(GameState::Restart),
            despawn_projectiles.in_set(RestartSystems::Combat),
        );
        app.add_systems(OnExit(AppState::Game), despawn_projectiles);
    }
}
