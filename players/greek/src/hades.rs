use {
    crate::prelude::*,
    greek_items::prelude::*,
    mythmallow::prelude::*,
};

/// Size of the player.
pub const SIZE: f32 = 20.00;

/// Color of the player.
pub const COLOR: Color = Color::BLACK;

/// Tag component for the player "Hades".
#[derive(Clone, Component, Debug, Reflect)]
pub struct Hades;

impl Playable for Hades {
    fn id(&self) -> SmolStr {
        "hades".into()
    }

    fn name(&self) -> SmolStr {
        "Hades".into()
    }

    fn collider(&self) -> Collider {
        Collider::ball(SIZE)
    }

    fn spawn(&self, world: &mut World) {
        world.run_system_once_with(self.clone(), spawn);
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
    In(player): In<Hades>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_action_input_map: Res<InputMap<GameAction>>,
    mut inventory: ResMut<Inventory>,
) {
    let mesh = MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(SIZE).into()).into(),
        material: materials.add(ColorMaterial::from(COLOR)),
        transform: Transform::from_translation(Vec3::new(0.00, 0.00, 2.00)),
        ..default()
    };

    PlayerBundle::builder()
        .player(player)
        .mesh(mesh)
        .input(game_action_input_map.clone())
        .build()
        .spawn(&mut commands);

    inventory.add(BidentOfHades.instantiate());
}
