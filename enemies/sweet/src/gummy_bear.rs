use {
    crate::prelude::*,
    mythmallow::{
        enemy::constants::MELEE_ENEMY_TAG,
        prelude::*,
    },
};

/// Size of the enemy.
pub const SIZE: f32 = 15.00;

/// Health of the enemy.
pub const HEALTH: Health = Health(5.00);

/// Speed of the enemy.
pub const SPEED: Speed = Speed(100.00);

/// Contact damage of the enemy.
pub const CONTACT_DAMAGE: f32 = 3.00;

/// Cooldown of contact damage of the enemy.
pub const CONTACT_DAMAGE_COOLDOWN: Duration = Duration::from_millis(1000);

/// Component for the enemy "Gummy Bear".
#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct GummyBear;

impl IEnemy for GummyBear {
    fn id(&self) -> SmolStr {
        "gummy-bear".into()
    }

    fn name(&self) -> SmolStr {
        "Gummy Bear".into()
    }

    fn contact_damage(&self) -> Option<(Damage, DamageCooldown)> {
        Some((Damage(CONTACT_DAMAGE), DamageCooldown::new(CONTACT_DAMAGE_COOLDOWN)))
    }

    fn health(&self) -> Health {
        HEALTH
    }

    fn speed(&self) -> Speed {
        SPEED
    }

    fn collider(&self) -> Collider {
        Collider::ball(SIZE)
    }

    fn spawn(&self, world: &mut World, position: Position) {
        world.run_system_once_with((self.clone(), position), spawn);
    }
}

/// Plugin for managing the enemy "Gummy Bear".
pub struct GummyBearPlugin;

impl Plugin for GummyBearPlugin {
    fn build(&self, app: &mut App) {
        // Register the enemy.
        let mut enemy_registry = app.world.resource_mut::<EnemyRegistry>();
        enemy_registry.register(SweetEnemyPack, GummyBear).add_tag(MELEE_ENEMY_TAG);

        // Register components.
        app.register_type::<GummyBear>();

        // Add systems.
        app.add_systems(Update, follow_player::<GummyBear>.in_set(GameplaySystems::Enemy));
    }
}

/// Spawns the enemy.
pub fn spawn(
    In((enemy, position)): In<(GummyBear, Position)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut counter: ResMut<EnemyCounter>,
) {
    let mesh = MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(SIZE).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(position.extend(Depth::Enemy.z())),
        ..default()
    };

    EnemyBundle::builder()
        .enemy(enemy)
        .position(position)
        .mesh(mesh)
        .build()
        .spawn(&mut commands, &mut counter);
}
