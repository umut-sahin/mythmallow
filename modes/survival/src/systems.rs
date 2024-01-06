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
    commands.insert_resource(WaveTimer::new(Duration::from_secs(10)));
}

/// Spawns the map.
pub fn spawn_map(mut commands: Commands) {
    commands.insert_resource(MAP_BOUNDS);
    commands.spawn((Name::new("Map"), Map, SpatialBundle::default())).with_children(|parent| {
        // Spawn horizontal lines.
        for i in 0..=GRID_SIZE {
            parent.spawn((
                Name::new(format!("Horizontal Line {}", i + 1)),
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        0.00,
                        (((GRID_SIZE as f32) / 2.00) - (i as f32)) * GRID_SPACING,
                        Depth::Map.z(),
                    )),
                    sprite: Sprite {
                        color: GRID_COLOR,
                        custom_size: Some(Vec2::new(GRID_SIZE as f32 * GRID_SPACING, GRID_WIDTH)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
        // Spawn vertical lines.
        for i in 0..=GRID_SIZE {
            parent.spawn((
                Name::new(format!("Vertical Line {}", i + 1)),
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        ((i as f32) - ((GRID_SIZE as f32) / 2.00)) * GRID_SPACING,
                        0.00,
                        Depth::Map.z(),
                    )),
                    sprite: Sprite {
                        color: GRID_COLOR,
                        custom_size: Some(Vec2::new(GRID_WIDTH, GRID_SIZE as f32 * GRID_SPACING)),
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
