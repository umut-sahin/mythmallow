use crate::prelude::*;


/// Selects the game mode when starting in game.
pub fn select_game_mode_when_starting_in_game(
    mut commands: Commands,
    args: ResMut<Args>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
    if game_mode_registry.is_empty() {
        return;
    }

    if let Some(specified_game_mode) = &args.start_in_game_mode {
        for (index, game_mode) in game_mode_registry.iter().enumerate() {
            if game_mode.name() == specified_game_mode {
                commands.insert_resource(GameModeIndex(index));
                break;
            }
        }
        return;
    }

    let selection = (0..game_mode_registry.len()).choose(rng.deref_mut()).unwrap();
    commands.insert_resource(GameModeIndex(selection));
}

/// Starts setting up the game when game mode is selected.
pub fn game_mode_selected(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_app_state.set(AppState::Game);
    next_game_state.set(GameState::Initialization);
}


/// Initializes the selected game mode.
pub fn initialize_game_mode(world: &mut World) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
    let selection = world.resource::<GameModeIndex>().0;
    game_mode_registry[selection].initialize(world);
}

/// Restarts up the selected game mode.
pub fn restart_game_mode(world: &mut World) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
    let selection = world.resource::<GameModeIndex>().0;
    game_mode_registry[selection].deinitialize(world);
}

/// Deinitializes the selected game mode.
pub fn deinitialize_game_mode(world: &mut World) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
    let selection = world.resource::<GameModeIndex>().0;
    game_mode_registry[selection].deinitialize(world);
    world.remove_resource::<GameModeIndex>();
}
