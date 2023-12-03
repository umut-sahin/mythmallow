use crate::{
    camera::{
        constants::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing the cameras.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<MainCamera>();

        // Set background color.
        app.insert_resource(ClearColor(BACKGROUND_COLOR));

        // Add systems.
        app.add_systems(Startup, spawn_main_camera);
        app.add_systems(OnEnter(GameState::Loading), player_lock.in_set(LoadingSystems::Camera));
        app.add_systems(
            PostUpdate,
            player_lock
                .in_set(GameplaySystems::Camera)
                .before(TransformSystem::TransformPropagate)
                .after(PhysicsSet::Sync),
        );
    }
}
