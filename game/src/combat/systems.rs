use crate::prelude::*;


/// Damages the player.
pub fn damage_player(
    mut commands: Commands,
    mut player_query: Query<&mut RemainingHealth, With<Player>>,
    player_hit_box_query: Query<&Parent, With<PlayerHitBox>>,
    player_damage_query: Query<
        (Entity, &Damage, Option<&DamageCooldown>),
        (With<DamagePlayerOnContact>, Without<Cooldown<Attack>>),
    >,
    mut collision_event_reader: EventReader<Collision>,
) {
    let mut apply_damage_if_applicable = |player_hit_box_entity, player_damage_entity| {
        let (damaging_entity, damage, damage_cooldown) =
            match player_damage_query.get(player_damage_entity) {
                Ok(query_result) => query_result,
                Err(_) => return,
            };
        let remaining_health = match player_hit_box_query.get(player_hit_box_entity) {
            Ok(parent) => player_query.get_mut(parent.get()),
            Err(_) => return,
        };
        if let Ok(mut remaining_health) = remaining_health {
            remaining_health.0 -= damage.0;
            if let Some(damage_cooldown) = damage_cooldown {
                commands
                    .entity(damaging_entity)
                    .insert(Cooldown::<Attack>::new(damage_cooldown.duration));
            }
        }
    };

    for Collision(contacts) in collision_event_reader.read().cloned() {
        apply_damage_if_applicable(contacts.entity1, contacts.entity2);
        apply_damage_if_applicable(contacts.entity2, contacts.entity1);
    }
}

/// Damages the enemies.
pub fn damage_enemies(
    mut commands: Commands,
    mut enemy_query: Query<&mut RemainingHealth, With<Enemy>>,
    enemy_hit_box_query: Query<&Parent, With<EnemyHitBox>>,
    enemy_damage_query: Query<
        (Entity, &Damage, Option<&DamageCooldown>),
        (With<DamageEnemiesOnContact>, Without<Cooldown<Attack>>),
    >,
    mut collision_event_reader: EventReader<Collision>,
) {
    let mut apply_damage_if_applicable = |enemy_hit_box_entity, enemy_damage_entity| {
        let (damaging_entity, damage, damage_cooldown) =
            match enemy_damage_query.get(enemy_damage_entity) {
                Ok(query_result) => query_result,
                Err(_) => return,
            };
        let remaining_health = match enemy_hit_box_query.get(enemy_hit_box_entity) {
            Ok(parent) => enemy_query.get_mut(parent.get()),
            Err(_) => return,
        };
        if let Ok(mut remaining_health) = remaining_health {
            remaining_health.0 -= damage.0;
            if let Some(damage_cooldown) = damage_cooldown {
                commands
                    .entity(damaging_entity)
                    .insert(Cooldown::<Attack>::new(damage_cooldown.duration));
            }
        }
    };

    for Collision(contacts) in collision_event_reader.read().cloned() {
        apply_damage_if_applicable(contacts.entity1, contacts.entity2);
        apply_damage_if_applicable(contacts.entity2, contacts.entity1);
    }
}


/// Handles player death.
pub fn player_death(
    mut commands: Commands,
    player_query: Query<&RemainingHealth, With<Player>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let remaining_health = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    if remaining_health.0 <= 0.00 {
        commands.insert_resource(GameResult::Lost);
        next_game_state.set(GameState::Over);
    }
}

/// Handles enemy death.
pub fn enemy_death(
    mut commands: Commands,
    enemy_query: Query<
        (
            Entity,
            &RemainingHealth,
            &Experience,
            &Position,
            &ExperiencePointVisuals,
            &ExperiencePointAttractionSpeed,
        ),
        With<Enemy>,
    >,
    map_query: Query<Entity, With<Map>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut experience_point_counter: ResMut<ExperiencePointCounter>,
) {
    for (
        enemy_entity,
        enemy_remaining_health,
        enemy_experience_reward,
        enemy_position,
        experience_point_visuals,
        experience_point_attraction_speed,
    ) in enemy_query.iter()
    {
        if enemy_remaining_health.0 <= 0.00 {
            if enemy_experience_reward.0 >= 0.00 {
                let mesh = MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(experience_point_visuals.size)).into(),
                    material: materials.add(ColorMaterial::from(experience_point_visuals.color)),
                    transform: Transform::from_translation(
                        enemy_position.extend(Depth::ExperiencePoint.z()),
                    ),
                    ..default()
                };
                let experience_point_bundle = ExperiencePointBundle {
                    position: *enemy_position,
                    attraction_speed: experience_point_attraction_speed.0.clone(),
                    mesh,
                    collider: Collider::circle(experience_point_visuals.size),
                    experience: *enemy_experience_reward,
                };
                let mut experience_point_entity =
                    experience_point_bundle.spawn(&mut commands, &mut experience_point_counter);
                experience_point_entity.set_parent(map_query.get_single().unwrap());
            }
            commands.entity(enemy_entity).despawn_recursive();
        }
    }
}


/// Despawns the projectiles on contact.
pub fn despawn_projectiles(
    mut commands: Commands,
    projectile_query: Query<Entity, With<Projectile>>,
    mut collision_started_event_reader: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collision_started_event_reader.read().cloned() {
        if projectile_query.get(entity1).is_ok() {
            commands.entity(entity1).despawn_recursive();
        } else if projectile_query.get(entity2).is_ok() {
            commands.entity(entity2).despawn_recursive();
        }
    }
}
