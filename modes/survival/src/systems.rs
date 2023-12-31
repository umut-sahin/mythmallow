use {
    crate::{
        constants::*,
        prelude::*,
    },
    mythmallow::prelude::*,
};


/// Initializes the game mode.
pub fn initialize(mut commands: Commands) {
    commands.insert_resource(CurrentWave(1));
}

/// Loads the current wave.
pub fn load(mut commands: Commands) {
    commands.insert_resource(WaveTimer(Timer::new(Duration::from_secs(5), TimerMode::Once)));
}


/// Spawns the map.
pub fn spawn_map(mut commands: Commands) {
    commands.insert_resource(MapBounds {
        x_min: -MAP_BOUND,
        x_max: MAP_BOUND,
        y_min: -MAP_BOUND,
        y_max: MAP_BOUND,
    });
    commands.spawn((Name::new("Map"), Map, SpatialBundle::default())).with_children(|parent| {
        // Define physics layers of the walls.
        let layers = CollisionLayers::new(
            [Layer::MapBound],
            [Layer::Player, Layer::Enemy, Layer::Projectile],
        );
        // Spawn left wall.
        parent.spawn((
            Name::new("Left Wall"),
            MapBound,
            RigidBody::Static,
            Collider::cuboid(50.0, MAP_BOUND * 2.0),
            layers,
            Position(Vector::NEG_X * (MAP_BOUND + 25.0)),
        ));
        // Spawn top wall.
        parent.spawn((
            Name::new("Top Wall"),
            MapBound,
            RigidBody::Static,
            Collider::cuboid(MAP_BOUND * 2.0, 50.0),
            layers,
            Position(Vector::Y * (MAP_BOUND + 25.0)),
        ));
        // Spawn right wall.
        parent.spawn((
            Name::new("Right Wall"),
            MapBound,
            RigidBody::Static,
            Collider::cuboid(50.0, MAP_BOUND * 2.0),
            layers,
            Position(Vector::X * (MAP_BOUND + 25.0)),
        ));
        // Spawn bottom wall.
        parent.spawn((
            Name::new("Bottom Wall"),
            MapBound,
            RigidBody::Static,
            Collider::cuboid(MAP_BOUND * 2.0, 50.0),
            layers,
            Position(Vector::NEG_Y * (MAP_BOUND + 25.0)),
        ));
        // Spawn horizontal lines.
        for i in 0..=MAP_SIZE {
            parent.spawn((
                Name::new(format!("Horizontal Line {}", i + 1)),
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        (((MAP_SIZE as f32) / 2.0) - (i as f32)) * GRID_SPACING,
                        0.0,
                    )),
                    sprite: Sprite {
                        color: Color::rgb(0.27, 0.27, 0.27),
                        custom_size: Some(Vec2::new(MAP_SIZE as f32 * GRID_SPACING, GRID_WIDTH)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
        // Spawn vertical lines.
        for i in 0..=MAP_SIZE {
            parent.spawn((
                Name::new(format!("Vertical Line {}", i + 1)),
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        ((i as f32) - ((MAP_SIZE as f32) / 2.0)) * GRID_SPACING,
                        0.0,
                        0.0,
                    )),
                    sprite: Sprite {
                        color: Color::rgb(0.27, 0.27, 0.27),
                        custom_size: Some(Vec2::new(GRID_WIDTH, MAP_SIZE as f32 * GRID_SPACING)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    });
}

/// Ticks wave timer and wins the current wave when wave timer is finished.
pub fn tick(
    time: Res<Time>,
    mut wave_timer: ResMut<WaveTimer>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    wave_timer.tick(time.delta());
    if wave_timer.just_finished() {
        next_game_state.set(GameState::Won);
    }
}

/// Wins the current wave.
pub fn win(mut commands: Commands, mut next_game_state: ResMut<NextState<GameState>>) {
    commands.insert_resource(GameResult::Won);
    next_game_state.set(GameState::Over);
}


/// Unloads the current wave.
pub fn unload(mut commands: Commands) {
    commands.remove_resource::<WaveTimer>();
}

/// Deinitializes the game mode.
pub fn deinitialize(mut commands: Commands) {
    commands.remove_resource::<CurrentWave>();
}
