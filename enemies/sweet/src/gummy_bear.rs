use {
    crate::prelude::*,
    mythmallow::{
        enemy::constants::MELEE_ENEMY_TAG,
        prelude::*,
    },
};

/// Tag component for the enemy "Gummy Bear".
#[derive(Component, Debug, Reflect)]
pub struct GummyBear;

impl Munchie for GummyBear {
    fn id(&self) -> SmolStr {
        "gummy-bear".into()
    }

    fn name(&self) -> SmolStr {
        "Gummy Bear".into()
    }

    fn collider(&self) -> Collider {
        Collider::ball(SIZE)
    }

    fn spawn(&self, world: &mut World, position: Position) {
        world.run_system_once_with(position, spawn);
    }
}

/// Plugin for managing the enemy "Gummy Bear".
pub struct GummyBearPlugin;

impl Plugin for GummyBearPlugin {
    fn build(&self, app: &mut App) {
        // Register the enemy.
        let mut enemy_registry = ENEMY_REGISTRY.lock().unwrap();
        enemy_registry.register(SweetMunchiesPack, GummyBear).add_tag(MELEE_ENEMY_TAG);
        drop(enemy_registry);

        // Register components.
        app.register_type::<GummyBear>();

        // Add systems.
        app.add_systems(Update, follow_player::<GummyBear>.in_set(GameplaySystems::Enemy));
    }
}

/// Size of the enemy.
pub const SIZE: f32 = 15.00;

/// Initial damage of the enemy.
pub const INITIAL_DAMAGE: f32 = 3.00;

/// Initial health of the enemy.
pub const INITIAL_HEALTH: f32 = 5.00;

/// Initial speed of the enemy.
pub const INITIAL_SPEED: f32 = 100.00;

/// Cooldown of applying damage to the player when in contact.
pub const DAMAGE_COOLDOWN: Duration = Duration::from_millis(1000);

/// Spawns the enemy.
pub fn spawn(
    In(position): In<Position>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut counter: ResMut<EnemyCounter>,
) {
    counter.increment();
    commands
        .spawn((
            // Tag
            GummyBear,
            // Attack
            DamagePlayerOnContact,
            Damage(INITIAL_DAMAGE),
            DamageCooldown::new(DAMAGE_COOLDOWN),
            // Enemy
            EnemyBundle {
                // Tags
                name: Name::new(format!("Enemy {} [Gummy Bear]", counter.get())),
                tag: Enemy,
                // Properties
                health: Health(INITIAL_HEALTH),
                speed: Speed(INITIAL_SPEED),
                // Combat
                remaining_health: RemainingHealth(INITIAL_HEALTH),
                // Physics
                body: RigidBody::Dynamic,
                restitution: Restitution::PERFECTLY_INELASTIC,
                position,
                collider: GummyBear.collider(),
                layers: CollisionLayers::new(
                    [Layer::Enemy, Layer::DamagePlayer],
                    [Layer::MapBound, Layer::Enemy, Layer::PlayerHitBox],
                ),
                // Texture
                mesh: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(SIZE).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(position.x, position.y, 1.00)),
                    ..default()
                },
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Hit Box"),
                EnemyHitBox,
                Sensor,
                Collider::ball(SIZE),
                CollisionLayers::new([Layer::EnemyHitBox], [Layer::DamageEnemies]),
            ));
        });
}
