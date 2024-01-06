use crate::prelude::*;


/// Initializes the selected game mode.
pub fn initialize_game_mode(world: &mut World) {
    let game_mode_registry = world.resource::<GameModeRegistry>();
    let selected_game_mode_index = world.resource::<SelectedGameModeIndex>();
    game_mode_registry[*selected_game_mode_index].clone().initialize(world);
}

/// Restarts up the selected game mode.
pub fn restart_game_mode(world: &mut World) {
    let game_mode_registry = world.resource::<GameModeRegistry>();
    let selected_game_mode_index = world.resource::<SelectedGameModeIndex>();
    game_mode_registry[*selected_game_mode_index].clone().deinitialize(world);
}

/// Deinitializes the selected game mode.
pub fn deinitialize_game_mode(world: &mut World) {
    let game_mode_registry = world.resource::<GameModeRegistry>();
    let selected_game_mode_index = world.resource::<SelectedGameModeIndex>();
    game_mode_registry[*selected_game_mode_index].clone().deinitialize(world);

    world.remove_resource::<SelectedGameModeIndex>();
}
