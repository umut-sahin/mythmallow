use {
    crate::constants::*,
    mythmallow::prelude::*,
};

/// Base range of the item.
pub const BASE_RANGE: f32 = 250.00;

/// Base damage of the item.
pub const BASE_DAMAGE: Damage = Damage(5.00);

/// Cooldown duration for the attack with the item.
pub const ATTACK_COOLDOWN: Duration = Duration::from_millis(600);

/// Size of the projectiles of the item.
pub const PROJECTILE_SIZE: f32 = 3.00;

/// Base speed for the projectiles of the item.
pub const BASE_PROJECTILE_SPEED: f32 = 200.00;

/// Tag component for the item "Bow of Artemis".
#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct BowOfArtemis;

impl IItem for BowOfArtemis {
    fn id(&self) -> SmolStr {
        "bow-of-artemis".into()
    }

    fn name(&self) -> SmolStr {
        "Bow of Artemis".into()
    }

    fn instantiate(&self) -> ItemInstance {
        ItemInstance::new(self.clone())
    }

    fn acquire(&self, world: &mut World) -> Entity {
        world.run_system_once_with(self.clone(), acquire)
    }

    fn release(&self, world: &mut World, entity: Entity) {
        world.run_system_once_with(entity, release);
    }
}

/// Plugin for managing the item "Bow of Artemis".
pub struct BowOfArtemisPlugin;

impl Plugin for BowOfArtemisPlugin {
    fn build(&self, app: &mut App) {
        // Register the item.
        let mut item_registry = ITEM_REGISTRY.lock().unwrap();
        item_registry.register(BowOfArtemis).add_tag(GREEK_ITEM_TAG);
        drop(item_registry);

        // Register resources.
        app.register_type::<BowOfArtemis>();

        // Add systems.
        app.add_systems(Update, attack.in_set(GameplaySystems::Item));
    }
}

/// Acquires the item.
pub fn acquire(
    In(item): In<BowOfArtemis>,
    mut commands: Commands,
    inventory: Res<Inventory>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Entity {
    commands
        .spawn((
            Name::new(format!("Item {} ({})", inventory.items.len(), item.name().to_string())),
            item,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.00).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(10.00, 0.00, Depth::Item.z())),
                ..default()
            },
        ))
        .id()
}

/// Releases the item.
pub fn release(In(entity): In<Entity>, mut commands: Commands) {
    if let Some(entity) = commands.get_entity(entity) {
        entity.despawn_recursive();
    }
}

/// Attacks with the item.
pub fn attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    item_query: Query<(Entity, &GlobalTransform), (With<BowOfArtemis>, Without<Cooldown<Attack>>)>,
    enemy_hit_box_query: Query<&Position, With<EnemyHitBox>>,
    spatial_query: SpatialQuery,
) {
    let damage = BASE_DAMAGE;
    let projectile_speed = BASE_PROJECTILE_SPEED;
    let attack_area = Collider::ball(BASE_RANGE);
    for (item_entity, &item_transform) in item_query.iter() {
        let item_position = Position(item_transform.translation().xy());

        let intersections = spatial_query.shape_intersections(
            &attack_area,
            item_position.xy(),
            0.00,
            SpatialQueryFilter::new().with_masks([Layer::EnemyHitBox]),
        );

        let mut enemies_in_range = intersections
            .iter()
            .filter_map(|&enemy_hit_box_entity| {
                enemy_hit_box_query
                    .get(enemy_hit_box_entity)
                    .map(|&enemy_hit_box_position| {
                        (enemy_hit_box_position, enemy_hit_box_position.distance(*item_position))
                    })
                    .ok()
            })
            .collect::<Vec<_>>();

        enemies_in_range.sort_by(|(_, distance1), (_, distance2)| {
            distance1.partial_cmp(distance2).unwrap_or(Ordering::Equal)
        });

        for (enemy_position, enemy_distance) in enemies_in_range {
            let enemy_direction = (enemy_position.xy() - item_position.xy()).normalize();
            let obstacle_between_item_and_enemy = spatial_query.cast_ray(
                *item_position,
                enemy_direction,
                enemy_distance,
                false,
                SpatialQueryFilter::new().with_masks([Layer::MapObstacle]),
            );
            if obstacle_between_item_and_enemy.is_none() {
                let mut item_entity_commands = commands.entity(item_entity);
                item_entity_commands.with_children(|parent| {
                    parent.spawn((
                        Name::new("Projectile"),
                        DamageEnemiesOnContact,
                        ProjectileBundle {
                            // Tags
                            tag: Projectile,
                            // Properties
                            damage,
                            // Physics
                            body: RigidBody::Dynamic,
                            position: item_position,
                            velocity: LinearVelocity(enemy_direction * projectile_speed),
                            collider: Collider::ball(PROJECTILE_SIZE),
                            layers: CollisionLayers::new(
                                [Layer::Projectile, Layer::DamageEnemies],
                                [Layer::MapBound, Layer::MapObstacle, Layer::EnemyHitBox],
                            ),
                            // Texture
                            mesh: MaterialMesh2dBundle {
                                mesh: meshes.add(shape::Circle::new(PROJECTILE_SIZE).into()).into(),
                                material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
                                transform: Transform::from_translation(
                                    item_position.extend(Depth::Projectile.z()),
                                ),
                                ..default()
                            },
                        },
                    ));
                });
                item_entity_commands.insert(Cooldown::<Attack>::new(ATTACK_COOLDOWN));
                break;
            }
        }
    }
}
