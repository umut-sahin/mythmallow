use crate::{
    enemy::constants::*,
    prelude::*,
};


/// Spawns some enemies.
pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let positions = [(200.00, 0.00), (0.00, 200.00), (-200.00, 0.00), (0.00, -200.00)];
    for (i, (x, y)) in positions.iter().cloned().enumerate() {
        commands.spawn((
            // Tags
            Name::new(format!("Enemy {i}")),
            Enemy,
            // Properties
            Damage(INITIAL_ENEMY_DAMAGE),
            Health(INITIAL_ENEMY_HEALTH),
            Speed(INITIAL_ENEMY_SPEED),
            // Combat
            RemainingHealth(INITIAL_ENEMY_HEALTH),
            // Physics
            RigidBody::Dynamic,
            Restitution::PERFECTLY_INELASTIC,
            Position(Vector::new(x, y)),
            Collider::ball(ENEMY_SIZE),
            CollisionLayers::new([Layer::Enemy], [Layer::Enemy]),
            // Texture
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ENEMY_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(x, y, 1.00)),
                ..default()
            },
        ));
    }
}

/// Despawns the enemies.
pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in &enemy_query {
        commands.entity(entity).despawn_recursive();
    }
}


/// Makes the enemies follow the player.
pub fn follow_player(
    mut enemy_query: Query<(&Position, &Speed, &mut LinearVelocity), With<Enemy>>,
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
