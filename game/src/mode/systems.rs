use crate::prelude::*;


/// Initializes the selected game mode.
pub fn initialize_game_mode(world: &mut World) {
    let game_mode_registry = world.resource::<GameModeRegistry>();
    let selected_game_mode_index = world.resource::<SelectedGameModeIndex>();

    let selected_game_mode = game_mode_registry[*selected_game_mode_index].clone();
    selected_game_mode.initialize(world);

    let market_can_be_opened_by_player = selected_game_mode.market_can_be_opened_by_player();
    let selected_game_mode_id = selected_game_mode.id();

    let mut market_configuration = world.resource_mut::<MarketConfiguration>();
    market_configuration.can_be_opened_by_player = market_can_be_opened_by_player;

    world.insert_resource(SelectedGameModeId(selected_game_mode_id));
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
    world.remove_resource::<SelectedGameModeId>();
}
