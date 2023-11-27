use crate::prelude::*;


/// Spawns the game mode selection screen.
pub fn spawn_game_mode_selection_screen(mut commands: Commands) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();

    if game_mode_registry.is_empty() {
        drop(game_mode_registry);
        // TODO: Replace panic with a proper error communicated through the UI.
        panic!("no game modes are available");
    }

    if game_mode_registry.len() == 1 {
        let selection_index = SelectedGameModeIndex(0);
        let selection = SelectedGameMode(Arc::clone(&game_mode_registry[selection_index]));

        commands.insert_resource(selection_index);
        commands.insert_resource(selection);

        return;
    }

    drop(game_mode_registry);
    // TODO: Add support for multiple game modes and a nice game mode selection screen.
    panic!("multiple game modes are not supported at the moment")
}

/// Despawns the game mode selection screen.
pub fn despawn_game_mode_selection_screen(
    mut commands: Commands,
    game_mode_selection_screen_query: Query<Entity, With<GameModeSelectionScreen>>,
) {
    if let Ok(entity) = game_mode_selection_screen_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Transitions to the player selection screen.
pub fn game_mode_selected(mut next_app_state: ResMut<NextState<AppState>>) {
    next_app_state.set(AppState::PlayerSelectionScreen);
}


/// Selects the game mode randomly or from the arguments of the application.
pub fn select_game_mode_when_starting_in_game(
    mut commands: Commands,
    args: ResMut<Args>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
    match &args.start_in_game_mode {
        Some(specified_game_mode_id) => {
            for (index, game_mode) in game_mode_registry.iter().enumerate() {
                if game_mode.id() == specified_game_mode_id {
                    log::info!("selected manually specified {:?} game mode", game_mode.id());

                    let selection_index = SelectedGameModeIndex(index);
                    let selection =
                        SelectedGameMode(Arc::clone(&game_mode_registry[selection_index]));

                    commands.insert_resource(selection_index);
                    commands.insert_resource(selection);

                    return;
                }
            }

            log::error!(
                "couldn't select manually specified {:?} game mode as it isn't registered",
                specified_game_mode_id,
            );
        },
        None => {
            if game_mode_registry.is_empty() {
                log::error!(
                    "couldn't select the game mode randomly as no game modes are registered",
                );
                return;
            }

            let selection_index = SelectedGameModeIndex(
                (0..game_mode_registry.len()).choose(rng.deref_mut()).unwrap(),
            );
            let selection = SelectedGameMode(Arc::clone(&game_mode_registry[selection_index]));

            log::info!("randomly selected {:?} game mode", selection.name());

            commands.insert_resource(selection_index);
            commands.insert_resource(selection);
        },
    }
}
