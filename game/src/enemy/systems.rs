use crate::prelude::*;


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
                spawn_enemy(world, &map_bounds, spawn);
            }
            continue;
        }

        if let Some(repeat) = &mut spawn.repeat {
            repeat.tick(time.delta());
            for _ in 0..repeat.times_finished_this_tick() {
                spawn_enemy(world, &map_bounds, spawn);
            }
        }
    }
}

/// Spawn an enemy.
pub fn spawn_enemy(world: &mut World, map_bounds: &MapBounds, spawn: &EnemySpawn) {
    let enemy = &spawn.enemy;
    let direction = &spawn.direction;

    let player_position = *world.query_filtered::<&Position, With<Player>>().single(world);
    let group_position = match spawn.position {
        EnemySpawnPosition::At(position) => position,
        EnemySpawnPosition::AroundPlayer { .. } => {
            todo!()
        },
        EnemySpawnPosition::Random => {
            let mut rng = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
            let x = match direction {
                EnemySpawnDirection::Any
                | EnemySpawnDirection::Above
                | EnemySpawnDirection::Below => {
                    rng.gen_range((map_bounds.x_min)..(map_bounds.x_max))
                },

                EnemySpawnDirection::Left => rng.gen_range((map_bounds.x_min)..(player_position.x)),
                EnemySpawnDirection::Right => {
                    rng.gen_range((player_position.x)..(map_bounds.x_max))
                },

                EnemySpawnDirection::Between(_, _) => todo!(),
            };
            let y = match direction {
                EnemySpawnDirection::Any
                | EnemySpawnDirection::Left
                | EnemySpawnDirection::Right => {
                    rng.gen_range((map_bounds.y_min)..(map_bounds.y_max))
                },

                EnemySpawnDirection::Above => {
                    rng.gen_range((player_position.y)..(map_bounds.y_max))
                },
                EnemySpawnDirection::Below => {
                    rng.gen_range((map_bounds.y_min)..(player_position.y))
                },

                EnemySpawnDirection::Between(_, _) => todo!(),
            };
            Position::new(Vector::new(x, y))
        },
    };

    for _ in 0..spawn.count {
        let mut position = group_position;

        let spread_x = spawn.spread.x_min != spawn.spread.x_max;
        let spread_y = spawn.spread.y_min != spawn.spread.y_max;
        if spread_x || spread_y {
            let mut rng = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
            if spread_x {
                position.x += rng.gen_range(spawn.spread.x_min..spawn.spread.x_max);
            }
            if spread_y {
                position.y += rng.gen_range(spawn.spread.y_min..spawn.spread.y_max);
            }
        }

        enemy.spawn(world, position);
    }
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
