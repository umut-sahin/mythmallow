use crate::prelude::*;


/// Sets up the main camera.
pub fn setup(mut commands: Commands) {
    commands.spawn((Name::new("Main Camera"), MainCamera, Camera2dBundle::default()));
}


/// Makes the main camera follow the player.
pub fn follow_player(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Position, With<Player>>,
) {
    if let Ok(player_position) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_position.x;
        camera_transform.translation.y = player_position.y;
    }
}
