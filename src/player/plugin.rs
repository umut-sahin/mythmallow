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

        app.add_systems(OnEnter(AppState::Game), spawn_player);
        app.add_systems(PreUpdate, cooldown::<Dashing>.in_set(GamePlaySystems::Player));
        app.add_systems(Update, (movement, dash).in_set(GamePlaySystems::Player));
        app.add_systems(PostUpdate, pause.in_set(GamePlaySystems::Player));
        app.add_systems(OnExit(AppState::Game), despawn_player);
    }
}
