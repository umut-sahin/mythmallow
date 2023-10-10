use crate::{
    map::constants::*,
    prelude::*,
};


/// Spawns the map.
pub fn spawn_map(mut commands: Commands) {
    commands.spawn((Name::new("Map"), Map, SpatialBundle::default())).with_children(|parent| {
        // horizontal lines
        for i in 0..=MAP_SIZE {
            parent.spawn((
                Name::new(format!("Horizontal Line {}", i + 1)),
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        (((MAP_SIZE as f32) / 2.0) - (i as f32)) * GRID_SPACING,
                        0.0,
                    )),
                    sprite: Sprite {
                        color: Color::rgb(0.27, 0.27, 0.27),
                        custom_size: Some(Vec2::new(MAP_SIZE as f32 * GRID_SPACING, GRID_WIDTH)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }

        // vertical lines
        for i in 0..=MAP_SIZE {
            parent.spawn((
                Name::new(format!("Vertical Line {}", i + 1)),
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        ((i as f32) - ((MAP_SIZE as f32) / 2.0)) * GRID_SPACING,
                        0.0,
                        0.0,
                    )),
                    sprite: Sprite {
                        color: Color::rgb(0.27, 0.27, 0.27),
                        custom_size: Some(Vec2::new(GRID_WIDTH, MAP_SIZE as f32 * GRID_SPACING)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    });
}

/// Despawns the map.
pub fn despawn_map(mut commands: Commands, map_query: Query<Entity, With<Map>>) {
    if let Ok(entity) = map_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
