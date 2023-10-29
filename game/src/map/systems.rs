use crate::prelude::*;


/// Despawns the map.
pub fn despawn_map(mut commands: Commands, map_query: Query<Entity, With<Map>>) {
    commands.remove_resource::<MapBounds>();
    if let Ok(entity) = map_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
