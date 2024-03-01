use crate::{
    map::constants::*,
    prelude::*,
};


/// Spawns the bounds of the map.
pub fn spawn_map_bounds(
    mut commands: Commands,
    map_bounds: Res<MapBounds>,
    map_query: Query<Entity, With<Map>>,
) {
    let mut map = match map_query.get_single() {
        Ok(map_entity) => commands.entity(map_entity),
        Err(_) => return,
    };
    map.with_children(|parent| {
        let layers = CollisionLayers::new(
            [Layer::MapBound],
            [Layer::Player, Layer::Enemy, Layer::Projectile],
        );

        let x_length = (map_bounds.x_max - map_bounds.x_min) + (4.00 * BOUND_THICKNESS);
        let y_length = map_bounds.y_max - map_bounds.y_min;

        parent.spawn((
            Name::new("Left Bound"),
            MapBound,
            RigidBody::Static,
            Collider::rectangle(2.00 * BOUND_THICKNESS, y_length),
            layers,
            Position(Vector::X * (map_bounds.x_min - BOUND_THICKNESS)),
        ));
        parent.spawn((
            Name::new("Top Bound"),
            MapBound,
            RigidBody::Static,
            Collider::rectangle(x_length, 2.00 * BOUND_THICKNESS),
            layers,
            Position(Vector::Y * (map_bounds.y_max + BOUND_THICKNESS)),
        ));
        parent.spawn((
            Name::new("Right Bound"),
            MapBound,
            RigidBody::Static,
            Collider::rectangle(2.00 * BOUND_THICKNESS, y_length),
            layers,
            Position(Vector::X * (map_bounds.x_max + BOUND_THICKNESS)),
        ));
        parent.spawn((
            Name::new("Bottom Bound"),
            MapBound,
            RigidBody::Static,
            Collider::rectangle(x_length, 2.00 * BOUND_THICKNESS),
            layers,
            Position(Vector::Y * (map_bounds.y_min - BOUND_THICKNESS)),
        ));
    });
}


/// Despawns the map.
pub fn despawn_map(mut commands: Commands, map_query: Query<Entity, With<Map>>) {
    commands.remove_resource::<MapBounds>();
    if let Ok(entity) = map_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
