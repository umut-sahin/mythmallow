use {
    super::resources::*,
    crate::prelude::*,
};


/// Sets up the game mode.
pub fn setup(mut commands: Commands) {
    commands.insert_resource(CurrentWave(1));
}

/// Loads the current wave.
pub fn load(mut commands: Commands) {
    commands.insert_resource(WaveTimer(Timer::new(Duration::from_secs(5), TimerMode::Once)));
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
