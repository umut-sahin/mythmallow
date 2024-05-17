use {
    crate::{
        constants::*,
        prelude::*,
    },
    mythmallow::prelude::*,
};


/// Initializes the game mode.
pub fn initialize(mut commands: Commands) {
    commands.insert_resource(WaveDurations::new(WAVES));
    commands.insert_resource(CurrentWave::default());
}

/// Selects the wave from the arguments of the survival game mode.
pub fn select_wave_when_starting_in_game(
    args: Res<Args>,
    survival_mode_args: Res<SurvivalModeArgs>,
    mut current_wave: ResMut<CurrentWave>,
) {
    if args.start_in_game {
        if let Some(wave) = &survival_mode_args.start_in_game_waves {
            *current_wave = CurrentWave(*wave);
        }
    }
}


/// Loads the current wave.
pub fn load(
    mut commands: Commands,
    current_wave: Res<CurrentWave>,
    wave_durations: Res<WaveDurations>,
) {
    commands.insert_resource(WaveTimer::new(
        wave_durations.get(current_wave.index()).copied().unwrap_or(Duration::ZERO),
    ));
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
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    wave_timer.tick(time.delta());
    if wave_timer.just_finished() {
        game_state_stack.transition(GameState::Won);
        next_game_state.set(GameState::Transition);
    }
}


/// Wins the current wave.
pub fn win(
    mut commands: Commands,
    mut current_wave: ResMut<CurrentWave>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if current_wave.is_last() {
        commands.insert_resource(GameResult::Won);
        game_state_stack.transition(GameState::Over);
        next_game_state.set(GameState::Transition);
    } else {
        current_wave.increment();
        game_state_stack.transition(GameState::Loading);
        next_game_state.set(GameState::Transition);
    }
}


/// Unloads the current wave.
pub fn unload(mut commands: Commands) {
    commands.remove_resource::<WaveTimer>();
}


/// Deinitializes the game mode.
pub fn deinitialize(mut commands: Commands) {
    commands.remove_resource::<CurrentWave>();
}
