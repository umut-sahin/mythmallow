use crate::{
    camera::{
        constants::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing the camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>();

        app.insert_resource(ClearColor(BACKGROUND_COLOR));

        app.add_systems(Startup, spawn_camera);
        app.add_systems(PostUpdate, follow_player.in_set(GameplaySystems::Camera));
    }
}
