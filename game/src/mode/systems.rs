use crate::prelude::*;


/// Initializes the selected game mode.
pub fn initialize_game_mode(world: &mut World) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
    let selection = world.resource::<GameModeIndex>();
    game_mode_registry[*selection].initialize(world);
}

/// Restarts up the selected game mode.
pub fn restart_game_mode(world: &mut World) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
    let selection = world.resource::<GameModeIndex>();
    game_mode_registry[*selection].deinitialize(world);
}

/// Deinitializes the selected game mode.
pub fn deinitialize_game_mode(world: &mut World) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
    let selection = world.resource::<GameModeIndex>();
    game_mode_registry[*selection].deinitialize(world);
    world.remove_resource::<GameModeIndex>();
}
