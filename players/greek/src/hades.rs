use {
    crate::prelude::*,
    greek_items::prelude::*,
    mythmallow::{
        player::constants::*,
        prelude::*,
    },
};

/// Tag component for the player "Hades".
#[derive(Component, Debug, Reflect)]
pub struct Hades;

impl Playable for Hades {
    fn id(&self) -> SmolStr {
        "hades".into()
    }

    fn name(&self) -> SmolStr {
        "Hades".into()
    }

    fn spawn(&self, world: &mut World) {
        world.run_system_once(spawn);
    }
}

/// Plugin for managing the player "Hades".
pub struct HadesPlugin;

impl Plugin for HadesPlugin {
    fn build(&self, app: &mut App) {
        // Register the player.
        let mut player_registry = PLAYER_REGISTRY.lock().unwrap();
        player_registry.register(GreekMythology, Hades);
        drop(player_registry);

        // Register components.
        app.register_type::<Hades>();
    }
}

/// Spawns the player "Hades".
pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_action_input_map: Res<InputMap<GameAction>>,
    mut inventory: ResMut<Inventory>,
) {
    commands
        .spawn((
            Hades,
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
                velocity: LinearVelocity(Vector::new(0.0, 0.0)),
                layers: CollisionLayers::new([Layer::Player], [Layer::Player]),
                mesh: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
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
            parent.spawn((Name::new("Hit Box"), PlayerHitBox, Sensor, Collider::ball(PLAYER_SIZE)));
        });

    inventory.add(BidentOfHades.instantiate());
}
