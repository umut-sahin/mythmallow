use {
    crate::prelude::*,
    mythmallow::{
        enemy::constants::RANGED_ENEMY_TAG,
        prelude::*,
    },
};

/// Size of the enemy.
pub const SIZE: f32 = 15.00;

/// Color of the enemy.
pub const COLOR: Color = Color::Rgba { red: 0.329, green: 0.149, blue: 0.031, alpha: 1.000 };

/// Health of the enemy.
pub const HEALTH: Health = Health(5.00);

/// Speed of the enemy.
pub const SPEED: Speed = Speed(80.00);

/// Damage of the enemy.
pub const DAMAGE: Damage = Damage(3.00);

/// Attack cooldown of the enemy.
pub const ATTACK_COOLDOWN: Duration = Duration::from_millis(1500);

/// Size of the projectiles of the enemy.
pub const PROJECTILE_SIZE: f32 = 7.50;

/// Color of the projectiles of the enemy.
pub const PROJECTILE_COLOR: Color =
    Color::Rgba { red: 0.229, green: 0.099, blue: 0.031, alpha: 1.000 };

/// Speed of the projectiles of the enemy.
pub const PROJECTILE_SPEED: f32 = 200.00;

/// Experience for defeating the enemy.
pub const EXPERIENCE_REWARD: Experience = Experience(3.00);

/// Component for the enemy "Chocolate Bar".
#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ChocolateBar;

impl IEnemy for ChocolateBar {
    fn id(&self) -> SmolStr {
        "chocolate-bar".into()
    }

    fn name(&self) -> SmolStr {
        "Chocolate Bar".into()
    }

    fn experience_reward(&self) -> Experience {
        EXPERIENCE_REWARD
    }

    fn health(&self) -> Health {
        HEALTH
    }

    fn speed(&self) -> Speed {
        SPEED
    }

    fn collider(&self) -> Collider {
        Collider::circle(SIZE)
    }

    fn spawn(&self, world: &mut World, position: Position) {
        world.run_system_once_with((self.clone(), position), spawn);
    }
}

/// Plugin for managing the enemy "Chocolate Bar".
pub struct ChocolateBarPlugin;

impl Plugin for ChocolateBarPlugin {
    fn build(&self, app: &mut App) {
        // Register the enemy.
        let mut enemy_registry = app.world.resource_mut::<EnemyRegistry>();
        enemy_registry.register(SweetEnemyPack, ChocolateBar).add_tag(RANGED_ENEMY_TAG);

        // Register components.
        app.register_type::<ChocolateBar>();

        // Add systems.
        app.add_systems(Update, attack.in_set(GameplaySystems::Enemy));
    }
}

/// Spawns the enemy.
pub fn spawn(
    In((enemy, position)): In<(ChocolateBar, Position)>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut counter: ResMut<EnemyCounter>,
) {
    let mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(SIZE)).into(),
        material: materials.add(ColorMaterial::from(COLOR)),
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
        .insert((
            AttractedTo(player_entity),
            IdealAttractionDistance(100.00),
            SlowdownOfGoingBackwardsDuringAttraction(0.75),
            Cooldown::<Attack>::new(ATTACK_COOLDOWN / 2),
        ));
}

/// Attacks to the player.
pub fn attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_query: Query<(Entity, &Transform), (With<ChocolateBar>, Without<Cooldown<Attack>>)>,
    player_query: Query<&Transform, (With<Player>, Without<ChocolateBar>)>,
    spatial_query: SpatialQuery,
) {
    let player_transform = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => {
            return;
        },
    };
    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let enemy_position = Position::new(enemy_transform.translation.xy());

        let to_player = (player_transform.translation - enemy_transform.translation).xy();
        let player_distance = to_player.length();
        let player_direction = to_player.normalize();

        let obstacle_between_enemy_and_player = utils::map::find_obstacle(
            &spatial_query,
            &enemy_position,
            &player_direction,
            player_distance,
        );
        if obstacle_between_enemy_and_player.is_some() {
            continue;
        }

        let projectile_entity = ProjectileBundle::builder()
            .mesh(MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(PROJECTILE_SIZE)).into(),
                material: materials.add(ColorMaterial::from(PROJECTILE_COLOR)),
                transform: Transform::from_translation(
                    enemy_position.extend(Depth::Projectile.z()),
                ),
                ..default()
            })
            .collider(Collider::circle(PROJECTILE_SIZE))
            .position(enemy_position)
            .velocity(LinearVelocity(player_direction * PROJECTILE_SPEED))
            .damage(DAMAGE)
            .build()
            .spawn_toward_player(&mut commands)
            .id();

        commands
            .entity(enemy_entity)
            .add_child(projectile_entity)
            .insert(Cooldown::<Attack>::new(ATTACK_COOLDOWN));
    }
}
