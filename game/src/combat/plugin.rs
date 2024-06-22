use crate::{
    combat::systems::*,
    prelude::*,
};

/// Plugin for managing the combat.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Attack>();
        app.register_type::<DamageCooldown>();
        app.register_type::<Cooldown<Attack>>();
        app.register_type::<Cooldown<Damage>>();
        app.register_type::<Originator>();
        app.register_type::<Projectile>();
        app.register_type::<RemainingHealth>();

        // Add systems.
        {
            app.add_systems(
                PreUpdate,
                (cooldown::<Attack>, cooldown::<Damage>).in_set(GameplaySystems::Combat),
            );

            app.add_systems(PostUpdate, start_attack_animations.in_set(GameplaySystems::Combat));
            app.add_systems(OnExit(GameState::Playing), pause_attack_animations);
            app.add_systems(
                Last,
                pause_attack_animations.run_if(|console_state: Res<ConsoleState>| {
                    console_state.is_changed() && console_state.open
                }),
            );
            app.add_systems(
                OnEnter(GameState::Playing),
                resume_attack_animations
                    .run_if(|console_state: Res<ConsoleState>| !console_state.open),
            );
            app.add_systems(
                Last,
                resume_attack_animations.run_if(
                    |game_state: Res<State<GameState>>, console_state: Res<ConsoleState>| {
                        console_state.is_changed()
                            && !console_state.open
                            && *game_state == GameState::Playing
                    },
                ),
            );

            app.add_systems(
                Update,
                (
                    damage_player_on_contact,
                    damage_player_on_contact_started,
                    damage_enemies_on_contact,
                    damage_enemies_on_contact_started,
                )
                    .in_set(GameplaySystems::Combat),
            );
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
}
