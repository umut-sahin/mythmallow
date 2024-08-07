use {
    crate::{
        constants::*,
        prelude::*,
    },
    mythmallow::item::constants::RANGED_ITEM_TAG,
};

/// Size of the item.
pub const SIZE: f32 = 5.00;

/// Color of the item.
pub const COLOR: Color = Color::srgb(0.00, 0.00, 1.00);

/// Base range of the item.
pub const BASE_RANGE: Range = Range(250.00);

/// Base damage of the item.
pub const BASE_DAMAGE: Damage = Damage(5.00);

/// Base cooldown duration of the attacks with the item.
pub const BASE_ATTACK_COOLDOWN: Duration = Duration::from_millis(600);

/// Size of the projectiles of the item.
pub const PROJECTILE_SIZE: f32 = 3.00;

/// Color of the projectiles of the item.
pub const PROJECTILE_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

/// Base speed for the projectiles of the item.
pub const BASE_PROJECTILE_SPEED: f32 = 200.00;

/// Base price of the item.
pub const BASE_PRICE: Balance = Balance(23.00);

/// Tag component for the item "Bow of Artemis".
#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct BowOfArtemis;

impl IItem for BowOfArtemis {
    fn id(&self) -> SmolStr {
        "bow-of-artemis".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "bow-of-artemis-name",
            args: smallvec![],
            fallback: "Bow of Artemis".into(),
        }
    }

    fn is_weapon(&self) -> bool {
        true
    }

    fn base_range(&self) -> Option<Range> {
        Some(BASE_RANGE)
    }

    fn base_price(&self) -> Balance {
        BASE_PRICE
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
        let mut item_registry = app.world_mut().resource_mut::<ItemRegistry>();
        item_registry.register(BowOfArtemis).add_tag(RANGED_ITEM_TAG).add_tag(GREEK_ITEM_TAG);

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
    let base_range = item.base_range().unwrap();
    commands
        .spawn((
            // Tags
            Name::new(format!("Item {} [{}]", inventory.items.len() + 1, item.id())),
            item,
            // Properties
            base_range,
            // Visuals
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(SIZE)).into(),
                material: materials.add(ColorMaterial::from(COLOR)),
                transform: Transform::from_translation(Vec3::new(0.00, 0.00, Depth::Item.z())),
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
    item_query: Query<
        (Entity, &GlobalTransform, &Range),
        (With<BowOfArtemis>, Without<Cooldown<Attack>>),
    >,
    enemy_hit_box_query: Query<&Position, With<EnemyHitBox>>,
    spatial_query: SpatialQuery,
) {
    for (item_entity, &item_transform, &item_range) in item_query.iter() {
        let item_position = Position(item_transform.translation().xy());
        let attack_area = Collider::circle(item_range.0);

        let enemies_in_range = utils::combat::find_enemies_in_range_sorted_by_distance(
            &spatial_query,
            &item_position,
            &attack_area,
            &enemy_hit_box_query,
        );
        for (_, enemy_position, enemy_distance) in enemies_in_range {
            let enemy_direction = (enemy_position.xy() - item_position.xy()).normalize();

            let obstacle_between_item_and_enemy = utils::map::find_obstacle(
                &spatial_query,
                &item_position,
                &enemy_direction,
                enemy_distance,
            );
            if obstacle_between_item_and_enemy.is_some() {
                continue;
            }

            ProjectileBundle::builder()
                .originator(item_entity)
                .mesh(MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(PROJECTILE_SIZE)).into(),
                    material: materials.add(ColorMaterial::from(PROJECTILE_COLOR)),
                    transform: Transform::from_translation(
                        item_position.extend(Depth::Projectile.z()),
                    ),
                    ..default()
                })
                .collider(Collider::circle(PROJECTILE_SIZE))
                .position(item_position)
                .velocity(LinearVelocity(enemy_direction * BASE_PROJECTILE_SPEED))
                .damage(BASE_DAMAGE)
                .build()
                .spawn_toward_enemies(&mut commands);

            commands.entity(item_entity).insert(Cooldown::<Attack>::new(BASE_ATTACK_COOLDOWN));

            break;
        }
    }
}
