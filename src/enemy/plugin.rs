use crate::{
    enemy::systems::*,
    prelude::*,
};

/// Plugin for managing enemies.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>();

        app.add_systems(OnEnter(AppState::Game), spawn_enemies);
        app.add_systems(Update, follow_player.in_set(GameplaySystems::Enemy));
        app.add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
