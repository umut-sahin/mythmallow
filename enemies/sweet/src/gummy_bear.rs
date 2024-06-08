use {
    crate::prelude::*,
    mythmallow::enemy::constants::MELEE_ENEMY_TAG,
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

/// Experience for defeating the enemy.
pub const EXPERIENCE_REWARD: Experience = Experience(5.00);

/// Component for the enemy "Gummy Bear".
#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct GummyBear;

impl IEnemy for GummyBear {
    fn id(&self) -> SmolStr {
        "gummy-bear".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "gummy-bear-name",
            args: smallvec![],
            fallback: "Gummy Bear".into(),
        }
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

    fn experience_reward(&self) -> Experience {
        EXPERIENCE_REWARD
    }

    fn collider(&self) -> Collider {
        Collider::circle(SIZE)
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
    }
}

/// Spawns the enemy.
pub fn spawn(
    In((enemy, position)): In<(GummyBear, Position)>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut counter: ResMut<EnemyCounter>,
) {
    let mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(SIZE)).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(position.extend(Depth::Enemy.z())),
        ..default()
    };

    let player_entity = player_query.get_single().unwrap();
    EnemyBundle::builder()
        .enemy(enemy)
        .position(position)
        .mesh(mesh)
        .build()
        .spawn(&mut commands, &mut counter)
        .insert((AttractedTo(player_entity), IdealAttractionDistance(25.00)));
}
