use crate::{
    enemy::constants::*,
    prelude::*,
};


/// Spawns an enemy.
pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        // Tags
        Name::new("Enemy 1"),
        Enemy,
        // Properties
        Damage(INITIAL_ENEMY_DAMAGE),
        Health(INITIAL_ENEMY_HEALTH),
        Speed(INITIAL_ENEMY_SPEED),
        // Combat
        RemainingHealth(INITIAL_ENEMY_HEALTH),
        // Physics
        PhysicsBundle::at(200.00, 0.00).with_collider(Collider { radius: ENEMY_SIZE }),
        // Texture
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(ENEMY_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(200.00, 0.00, 1.00)),
            ..default()
        },
    ));
    commands.spawn((
        // Tags
        Name::new("Enemy 2"),
        Enemy,
        // Properties
        Damage(INITIAL_ENEMY_DAMAGE),
        Health(INITIAL_ENEMY_HEALTH),
        Speed(INITIAL_ENEMY_SPEED),
        // Combat
        RemainingHealth(INITIAL_ENEMY_HEALTH),
        // Physics
        PhysicsBundle::at(-200.00, 0.00).with_collider(Collider { radius: ENEMY_SIZE }),
        // Texture
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(ENEMY_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(-200.00, 0.00, 1.00)),
            ..default()
        },
    ));
    commands.spawn((
        // Tags
        Name::new("Enemy 3"),
        Enemy,
        // Properties
        Damage(INITIAL_ENEMY_DAMAGE),
        Health(INITIAL_ENEMY_HEALTH),
        Speed(INITIAL_ENEMY_SPEED),
        // Combat
        RemainingHealth(INITIAL_ENEMY_HEALTH),
        // Physics
        PhysicsBundle::at(0.00, 200.00).with_collider(Collider { radius: ENEMY_SIZE }),
        // Texture
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(ENEMY_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(0.00, 200.00, 1.00)),
            ..default()
        },
    ));
}

/// Despawns the enemies.
pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in &enemy_query {
        commands.entity(entity).despawn_recursive();
    }
}


/// Makes the enemies follow the player.
pub fn follow_player(
    mut enemy_query: Query<(&Position, &Speed, &mut Velocity), With<Enemy>>,
    player_query: Query<&Position, (With<Player>, Without<Enemy>)>,
) {
    if let Ok(player_position) = player_query.get_single() {
        for (enemy_position, enemy_speed, mut enemy_velocity) in enemy_query.iter_mut() {
            let direction = player_position.0 - enemy_position.0;
            enemy_velocity.0 = if direction.length() > 25.00 {
                direction.normalize() * enemy_speed.0
            } else {
                Vec2::ZERO
            };
        }
    }
}
