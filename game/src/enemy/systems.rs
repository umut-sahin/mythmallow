use crate::{
    enemy::constants::*,
    prelude::*,
};


/// Initializes the enemy counter.
pub fn initialize_enemy_counter(mut commands: Commands) {
    commands.insert_resource(EnemyCounter::default());
}

/// Initializes the enemy spawn pattern.
pub fn initialize_enemy_spawn_pattern(world: &mut World) {
    let enemy_registry = ENEMY_REGISTRY.lock().unwrap();
    let selection = world.resource::<SelectedEnemyPackIndex>();
    let spawn_pattern = enemy_registry[*selection].0.spawn_pattern(world).unwrap_or_else(|| {
        let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
        let selected_game_mode_index = world.resource::<SelectedGameModeIndex>();
        game_mode_registry[*selected_game_mode_index].default_enemy_spawn_pattern(world)
    });
    log::info!("enemy spawn pattern for the level:\n{:#?}", spawn_pattern);
    world.insert_resource(spawn_pattern);
}


/// Spawns enemies according to the current enemy spawn pattern.
pub fn spawn_enemies(world: &mut World) {
    let time = *world.resource::<Time>();
    let map_bounds = *world.resource::<MapBounds>();

    let enemy_spawn_pattern = world.resource::<EnemySpawnPattern>().clone();
    let mut spawns = enemy_spawn_pattern.spawns.lock().unwrap();

    for spawn in spawns.iter_mut() {
        if !spawn.delay.finished() {
            spawn.delay.tick(time.delta());
            if spawn.delay.just_finished() {
                spawn.remaining += spawn.count;
                spawn_enemy(world, &map_bounds, spawn);
            } else {
                continue;
            }
        }

        if let Some(repeat) = &mut spawn.repeat {
            repeat.tick(time.delta());
            if repeat.finished() {
                let n = repeat.times_finished_this_tick();
                spawn.remaining += n * spawn.count;
                spawn_enemy(world, &map_bounds, spawn);
            }
        }

        if spawn.remaining != 0 {
            match spawn.interval.as_mut() {
                Some(interval) => {
                    interval.tick(time.delta());
                    for _ in 0..interval.times_finished_this_tick() {
                        spawn_enemy(world, &map_bounds, spawn);
                        if spawn.remaining == 0 {
                            break;
                        }
                    }
                },
                None => {
                    for _ in 0..spawn.remaining {
                        spawn_enemy(world, &map_bounds, spawn);
                    }
                },
            }

            if spawn.remaining == 0 {
                if let Some(interval) = spawn.interval.as_mut() {
                    interval.reset();
                }
                spawn.spawned = 0;
                break;
            }
        }
    }
}

/// Spawns a single enemy from the spawn pattern.
pub fn spawn_enemy(world: &mut World, map_bounds: &MapBounds, spawn: &mut EnemySpawn) {
    if spawn.spawned % spawn.count == 0 {
        let mut group_position = match spawn.position {
            EnemySpawnPosition::At(position) => position,
            _ => {
                let player_position =
                    match world.query_filtered::<&Position, With<Player>>().get_single(world) {
                        Ok(player_position) => *player_position,
                        Err(_) => return,
                    };

                let enemy_direction = {
                    let EnemySpawnDirection { from_degrees, to_degrees } = spawn.direction;
                    let rotation = if from_degrees == to_degrees {
                        Rotation::from_degrees(from_degrees)
                    } else {
                        let mut rng = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
                        Rotation::from_degrees(rng.gen_range(from_degrees..to_degrees))
                    };
                    rotation.rotate(Vector::X)
                };

                match spawn.position {
                    EnemySpawnPosition::At(_) => unreachable!(),
                    EnemySpawnPosition::AroundPlayer { near, far } => {
                        let mut rng = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
                        let distance = rng.gen_range(near..far);
                        Position::new(*player_position + (enemy_direction * distance))
                    },
                    EnemySpawnPosition::Random => {
                        fn find_distance_to_map_bounds(
                            In((origin, direction, max_distance)): In<(Position, Vector, f32)>,
                            spatial: SpatialQuery,
                        ) -> Option<f32> {
                            let ray = spatial.cast_ray(
                                *origin,
                                direction,
                                max_distance,
                                false,
                                SpatialQueryFilter::default().with_masks([Layer::MapBound]),
                            );
                            ray.map(|details| details.time_of_impact)
                        }

                        let max_x_distance = map_bounds.x_max - map_bounds.x_min;
                        let max_y_distance = map_bounds.y_max - map_bounds.y_min;
                        let max_distance = max_x_distance.powf(2.00) + max_y_distance.powf(2.00);

                        let distance_to_map_bounds = world.run_system_once_with(
                            (player_position, enemy_direction, max_distance),
                            find_distance_to_map_bounds,
                        );

                        match distance_to_map_bounds {
                            Some(distance) => {
                                let distance = if distance < MINIMUM_ENEMY_SPAWN_DISTANCE {
                                    distance
                                } else {
                                    let mut rng = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
                                    rng.gen_range(MINIMUM_ENEMY_SPAWN_DISTANCE..distance)
                                };
                                Position::new(*player_position + (enemy_direction * distance))
                            },
                            _ => {
                                let mut rng = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
                                let distance = rng.gen_range(
                                    MINIMUM_ENEMY_SPAWN_DISTANCE
                                        ..(3.00 * MINIMUM_ENEMY_SPAWN_DISTANCE),
                                );
                                Position::new(*player_position + (enemy_direction * distance))
                            },
                        }
                    },
                }
            },
        };

        group_position.x = group_position.x.clamp(map_bounds.x_min, map_bounds.x_max);
        group_position.y = group_position.y.clamp(map_bounds.y_min, map_bounds.y_max);

        spawn.group_position = group_position;
    }

    let mut enemy_position = spawn.group_position;
    enemy_position.x += {
        if spawn.spread.x_min == spawn.spread.x_max {
            spawn.spread.x_min
        } else {
            assert!(spawn.spread.x_min < spawn.spread.x_max);
            let mut rng = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
            rng.gen_range(spawn.spread.x_min..spawn.spread.x_max)
        }
    };
    enemy_position.y += {
        if spawn.spread.y_min == spawn.spread.y_max {
            spawn.spread.y_min
        } else {
            assert!(spawn.spread.y_min < spawn.spread.y_max);
            let mut rng = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
            rng.gen_range(spawn.spread.y_min..spawn.spread.y_max)
        }
    };

    enemy_position.x = enemy_position.x.clamp(map_bounds.x_min, map_bounds.x_max);
    enemy_position.y = enemy_position.y.clamp(map_bounds.y_min, map_bounds.y_max);

    let enemy = &spawn.enemy;

    let desired_enemy_transform =
        Transform::from_translation(enemy_position.extend(Depth::Enemy.z()));
    let found_enemy_transform = world
        .run_system_once_with((desired_enemy_transform, enemy.collider(), 0.25), find_free_space);

    if let Some(transform) = found_enemy_transform {
        enemy.spawn(world, Position::new(transform.translation.xy()));
        world.run_system_once(apply_deferred);
        spawn.remaining -= 1;
    }

    // Spawned is increased either way to change the group position in the next try.
    // This is to avoid repeatedly trying to spawn in the same non-spawnable position.
    spawn.spawned += 1;
}

/// Finds a free space to spawn an enemy.
pub fn find_free_space(
    In((target_transform, collider, margin)): In<(Transform, Collider, Scalar)>,
    mut spatial: SpatialQuery,
    query: Query<(&Collider, &Transform)>,
) -> Option<Transform> {
    spatial.update_pipeline();

    let mut target_position = target_transform.translation.truncate();
    let rotation = Rotation::from(target_transform.rotation);

    let mut collider = collider.clone();
    collider.set_scale(Vector::ONE + margin, 8);

    let filter = SpatialQueryFilter::default();
    for _ in 0..100 {
        let intersections = spatial.shape_intersections(
            &collider,
            target_position,
            rotation.as_radians(),
            filter.clone(),
        );

        if intersections.is_empty() {
            return Some(
                target_transform.with_translation(target_position.extend(Depth::Enemy.z())),
            );
        } else {
            for entity in intersections {
                let Ok((hit_collider, hit_transform)) = query.get(entity) else {
                    continue;
                };
                let hit_translation = hit_transform.translation.truncate();

                if let Ok(Some(contact)) = contact_query::contact(
                    &collider,
                    target_position,
                    rotation,
                    hit_collider,
                    hit_translation,
                    hit_transform.rotation,
                    0.0,
                ) {
                    let normal = contact.global_normal2(&hit_transform.rotation.into());
                    let delta = normal * (contact.penetration + 0.00001);
                    target_position += delta;
                }
            }
        }
    }

    None
}

/// Despawns the enemies.
pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in &enemy_query {
        commands.entity(entity).despawn_recursive();
    }
}


/// Clears the enemy counter.
pub fn clear_enemy_counter(mut commands: Commands) {
    commands.remove_resource::<EnemyCounter>();
}

/// Clears the enemy pack selection.
pub fn clear_enemy_pack_selection(mut commands: Commands) {
    commands.remove_resource::<SelectedEnemyPackIndex>();
    commands.remove_resource::<SelectedEnemyPack>();
}


/// Makes the enemies follow the player.
pub fn follow_player<T: Component>(
    mut enemy_query: Query<(&Position, &Speed, &mut LinearVelocity), With<T>>,
    player_query: Query<&Position, (With<Player>, Without<T>)>,
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
