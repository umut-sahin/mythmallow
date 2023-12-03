use crate::prelude::*;


/// Spawns the enemy selection screen.
pub fn spawn_enemy_selection_screen(mut commands: Commands) {
    let enemy_registry = ENEMY_REGISTRY.lock().unwrap();

    if enemy_registry.is_empty() {
        drop(enemy_registry);
        // TODO: Replace panic with a proper error communicated through the UI.
        panic!("no enemy packs are available");
    }

    if enemy_registry.len() == 1 {
        let selection_index = SelectedEnemyPackIndex(0);
        let selection = SelectedEnemyPack(enemy_registry[selection_index].clone());

        commands.insert_resource(selection_index);
        commands.insert_resource(selection);

        return;
    }

    drop(enemy_registry);
    // TODO: Add support for multiple enemy packs and a nice enemy selection screen.
    panic!("multiple enemy packs are not supported at the moment")
}

/// Despawns the enemy selection screen.
pub fn despawn_enemy_selection_screen(
    mut commands: Commands,
    enemy_selection_screen_query: Query<Entity, With<EnemySelectionScreen>>,
) {
    if let Ok(entity) = enemy_selection_screen_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Starts the game.
pub fn enemy_pack_selected(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_app_state.set(AppState::Game);
    next_game_state.set(GameState::Initialization);
}


/// Selects the enemies randomly or from the arguments of the application.
pub fn select_enemy_pack_when_starting_in_game(
    mut commands: Commands,
    args: ResMut<Args>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    let enemy_registry = ENEMY_REGISTRY.lock().unwrap();
    match &args.start_in_game_mode {
        Some(specified_enemy_pack_id) => {
            for (index, (enemy_pack, _)) in enemy_registry.iter().enumerate() {
                if enemy_pack.id() == specified_enemy_pack_id {
                    log::info!("selected manually specified {:?} enemies", enemy_pack.id());

                    let selection_index = SelectedEnemyPackIndex(index);
                    let selection = SelectedEnemyPack(enemy_registry[selection_index].clone());

                    commands.insert_resource(selection_index);
                    commands.insert_resource(selection);

                    return;
                }
            }

            log::error!(
                "couldn't select manually specified {:?} enemies as it isn't registered",
                specified_enemy_pack_id,
            );
        },
        None => {
            if enemy_registry.is_empty() {
                log::error!(
                    "couldn't select the enemies randomly as no enemy packs are registered",
                );
                return;
            }

            let selection_index =
                SelectedEnemyPackIndex((0..enemy_registry.len()).choose(rng.deref_mut()).unwrap());
            let selection = SelectedEnemyPack(enemy_registry[selection_index].clone());

            log::info!("randomly selected {:?} enemies", enemy_registry[selection_index].0.name());

            commands.insert_resource(selection_index);
            commands.insert_resource(selection);
        },
    }
}
