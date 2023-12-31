use {
    crate::prelude::*,
    greek_items::prelude::*,
    mythmallow::{
        player::constants::*,
        prelude::*,
    },
};

/// Tag component for the player "Artemis".
#[derive(Component, Debug, Reflect)]
pub struct Artemis;

impl Playable for Artemis {
    fn id(&self) -> SmolStr {
        "artemis".into()
    }

    fn name(&self) -> SmolStr {
        "Artemis".into()
    }

    fn spawn(&self, world: &mut World) {
        world.run_system_once(spawn);
    }
}

/// Plugin for managing the player "Artemis".
pub struct ArtemisPlugin;

impl Plugin for ArtemisPlugin {
    fn build(&self, app: &mut App) {
        // Register the player.
        let mut player_registry = PLAYER_REGISTRY.lock().unwrap();
        player_registry.register(GreekMythology, Artemis);
        drop(player_registry);

        // Register components.
        app.register_type::<Artemis>();
    }
}

/// Spawns the player "Artemis".
pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_action_input_map: Res<InputMap<GameAction>>,
    mut inventory: ResMut<Inventory>,
) {
    commands
        .spawn((
            Artemis,
            PlayerBundle {
                name: Name::new("Player"),
                tag: Player,
                damage: Damage(INITIAL_PLAYER_DAMAGE),
                health: Health(INITIAL_PLAYER_HEALTH),
                speed: Speed(INITIAL_PLAYER_SPEED),
                remaining_health: RemainingHealth(INITIAL_PLAYER_HEALTH),
                body: RigidBody::Dynamic,
                restitution: Restitution::PERFECTLY_INELASTIC,
                position: Position(Vector::new(0.00, 0.00)),
                collider: Collider::ball(PLAYER_SIZE),
                velocity: LinearVelocity(Vector::new(0.00, 0.00)),
                layers: CollisionLayers::new([Layer::Player], [Layer::MapBound]),
                axes: LockedAxes::ROTATION_LOCKED,
                mesh: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform::from_translation(Vec3::new(0.00, 0.00, 2.00)),
                    ..default()
                },
                input: InputManagerBundle::<GameAction> {
                    action_state: ActionState::default(),
                    input_map: game_action_input_map.clone(),
                },
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Hit Box"),
                PlayerHitBox,
                Sensor,
                Collider::ball(PLAYER_SIZE),
                CollisionLayers::new([Layer::PlayerHitBox], [Layer::DamagePlayer]),
            ));
        });

    inventory.add(BowOfArtemis.instantiate());
}
