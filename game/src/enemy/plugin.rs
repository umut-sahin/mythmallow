use crate::{
    enemy::systems::*,
    prelude::*,
};

/// Plugin for managing enemies.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Enemy>();

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Loading),
            (initialize_enemy_counter, initialize_enemy_spawn_pattern)
                .in_set(LoadingSystems::Enemy),
        );
        app.add_systems(Update, spawn_enemies.in_set(GameplaySystems::Enemy));
        app.add_systems(OnEnter(GameState::Won), (despawn_enemies, clear_enemy_counter));
        app.add_systems(OnEnter(GameState::Over), (despawn_enemies, clear_enemy_counter));
        app.add_systems(
            OnEnter(GameState::Restart),
            (despawn_enemies, clear_enemy_counter).in_set(RestartSystems::Enemy),
        );
        app.add_systems(
            OnExit(AppState::Game),
            (despawn_enemies, clear_enemy_counter, clear_enemy_pack_selection),
        );
    }
}
