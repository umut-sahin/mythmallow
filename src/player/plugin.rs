use crate::{
    player::systems::*,
    prelude::*,
    status_effect::systems::cooldown,
};

/// Plugin for managing the player.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
        app.register_type::<PlayerHitbox>();

        app.add_systems(
            OnEnter(GameState::Initialization),
            spawn_player.in_set(InitializationSystems::Player),
        );

        app.add_systems(PreUpdate, cooldown::<Dashing>.in_set(GameplaySystems::Player));
        app.add_systems(Update, (movement, dash).in_set(GameplaySystems::Player));
        app.add_systems(PostUpdate, pause_the_game.in_set(GameplaySystems::Player));

        app.add_systems(OnEnter(GameState::Over), despawn_player);
        app.add_systems(OnEnter(GameState::Restart), despawn_player.in_set(RestartSystems::Player));
        app.add_systems(OnExit(AppState::Game), despawn_player);
    }
}
