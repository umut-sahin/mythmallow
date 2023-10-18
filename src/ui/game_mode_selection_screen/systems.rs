use crate::prelude::*;


/// Spawns game mode selection screen.
pub fn spawn_game_mode_selection_screen(mut commands: Commands) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();

    if game_mode_registry.is_empty() {
        drop(game_mode_registry);
        // TODO: Replace this with a proper error communicated through the UI.
        panic!("no game modes are available");
    }

    if game_mode_registry.len() == 1 {
        commands.insert_resource(GameModeIndex(0));
        return;
    }

    drop(game_mode_registry);
    // TODO: Add support for multiple game modes and a nice game mode selection screen.
    panic!("multiple game modes are not supported at the moment")
}

/// Despawns game mode selection screen.
pub fn despawn_game_mode_selection_screen(
    mut commands: Commands,
    game_mode_selection_screen_query: Query<Entity, With<GameModeSelectionScreen>>,
) {
    if let Ok(entity) = game_mode_selection_screen_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
