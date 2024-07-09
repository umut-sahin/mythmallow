use {
    crate::{
        constants::*,
        prelude::*,
    },
    mythmallow::item::constants::MELEE_ITEM_TAG,
};

/// Vertices of the item, as the item is just a triangle for the time being.
pub const VERTICES: [Vec2; 3] =
    [Vec2::new(15.00, 0.00), Vec2::new(-10.00, 5.00), Vec2::new(-10.00, -5.00)];

/// Color of the item.
pub const COLOR: Color = Color::srgb(1.00, 0.65, 0.00);

/// Base damage of the item.
pub const BASE_DAMAGE: Damage = Damage(5.00);

/// Base range of the item.
pub const BASE_RANGE: Range = Range(100.00);

/// Base duration of the attacks with the item.
pub const BASE_ATTACK_DURATION: Duration = Duration::from_millis(240);

/// Base cooldown duration of the attacks with the item.
pub const BASE_ATTACK_COOLDOWN: Duration = Duration::from_millis(900);

/// Base price of the item.
pub const BASE_PRICE: Balance = Balance(20.00);

/// Tag component for the item "Bident of Hades".
#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct BidentOfHades;

impl IItem for BidentOfHades {
    fn id(&self) -> SmolStr {
        "bident-of-hades".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "bident-of-hades-name",
            args: smallvec![],
            fallback: "Bident of Hades".into(),
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

/// Plugin for managing the item "Bident of Hades".
pub struct BidentOfHadesPlugin;

impl Plugin for BidentOfHadesPlugin {
    fn build(&self, app: &mut App) {
        // Register the item.
        let mut item_registry = app.world_mut().resource_mut::<ItemRegistry>();
        item_registry.register(BidentOfHades).add_tag(MELEE_ITEM_TAG).add_tag(GREEK_ITEM_TAG);

        // Register components.
        app.register_type::<BidentOfHades>();

        // Add systems.
        app.add_systems(Update, attack.in_set(GameplaySystems::Item));
    }
}

/// Acquires the item.
pub fn acquire(
    In(item): In<BidentOfHades>,
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
                mesh: meshes.add(Triangle2d { vertices: VERTICES }).into(),
                material: materials.add(ColorMaterial::from(COLOR)),
                transform: Transform::from_translation(Vec3::new(0.00, 0.00, Depth::Item.z())),
                ..default()
            },
            // Combat
            DamageEnemiesOnContactStarted,
            BASE_DAMAGE,
            // Physics
            CollisionLayers::new([Layer::DamageEnemies], [Layer::EnemyHitBox]),
            Collider::triangle(VERTICES[0], VERTICES[1], VERTICES[2]),
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
    item_query: Query<
        (Entity, &GlobalTransform, &Range),
        (With<BidentOfHades>, Without<Attack>, Without<Cooldown<Attack>>),
    >,
    enemy_hit_box_query: Query<&Position, With<EnemyHitBox>>,
    spatial_query: SpatialQuery,
) {
    for (item_entity, item_global_transform, item_range) in item_query.iter() {
        let item_position = Position(item_global_transform.translation().xy());
        let attack_area = Collider::circle(item_range.0);

        let enemies_in_range = utils::combat::find_enemies_in_range_sorted_by_distance(
            &spatial_query,
            &item_position,
            &attack_area,
            &enemy_hit_box_query,
        );
        if enemies_in_range.is_empty() {
            continue;
        }

        let (_, closest_enemy_position, _) = enemies_in_range[0];

        let direction = (closest_enemy_position.xy() - item_position.xy()).normalize();
        let range = BASE_RANGE;
        let duration = BASE_ATTACK_DURATION;

        commands.entity(item_entity).insert((
            Attack::Thrust { direction, range, duration, started: false },
            Cooldown::<Attack>::new(BASE_ATTACK_COOLDOWN),
        ));
    }
}
