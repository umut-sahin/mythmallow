use crate::prelude::*;


/// Spawns the main camera.
pub fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((Name::new("Main Camera"), MainCamera, Camera2dBundle::default()));
}


/// Makes the main camera follow the player.
pub fn follow_player(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>, Without<MainCamera>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
