use {
    crate::{
        constants::*,
        prelude::*,
    },
    mythmallow::prelude::*,
};


/// Initializes the game mode.
pub fn initialize(mut commands: Commands) {
    commands.insert_resource(CurrentChapter(1));
}

/// Loads the current chapter.
pub fn load(mut commands: Commands) {
    commands
        .insert_resource(ChapterTimer(Timer::new(Duration::from_secs(60 * 5), TimerMode::Once)));
}


/// Spawns the map.
pub fn spawn_map(mut commands: Commands) {
    commands.insert_resource(MAP_BOUNDS);
    commands.spawn((Name::new("Map"), Map, SpatialBundle::default())).with_children(|_parent| {
        // TODO: generate and spawn the maze
    });
}

/// Ticks the chapter timer and loses the game if it's finished.
pub fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut chapter_timer: ResMut<ChapterTimer>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    chapter_timer.tick(time.delta());
    if chapter_timer.just_finished() {
        commands.insert_resource(GameResult::Lost);
        next_game_state.set(GameState::Over);
    }
}


/// Unloads the current chapter.
pub fn unload(mut commands: Commands) {
    commands.remove_resource::<ChapterTimer>();
}

/// Deinitializes the game mode.
pub fn deinitialize(mut commands: Commands) {
    commands.remove_resource::<CurrentChapter>();
}
