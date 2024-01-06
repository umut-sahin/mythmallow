use crate::prelude::*;


/// Spawns the enemy selection screen.
pub fn spawn_enemy_selection_screen(mut commands: Commands, enemy_registry: Res<EnemyRegistry>) {
    if enemy_registry.is_empty() {
        // TODO: Replace panic with a proper error communicated through the UI.
        panic!("no enemy packs are available");
    }

    if enemy_registry.len() == 1 {
        let selected_enemy_pack_index = SelectedEnemyPackIndex(0);
        commands.insert_resource(selected_enemy_pack_index);
        return;
    }

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
    enemy_registry: Res<EnemyRegistry>,
) {
    match &args.start_in_game_mode {
        Some(specified_enemy_pack_id) => {
            for (index, entry) in enemy_registry.iter().enumerate() {
                if entry.pack.id() == specified_enemy_pack_id {
                    log::info!("selected manually specified {:?} enemies", entry.pack.id());

                    let selected_enemy_pack_index = SelectedEnemyPackIndex(index);
                    commands.insert_resource(selected_enemy_pack_index);
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

            let selected_enemy_pack_index =
                SelectedEnemyPackIndex((0..enemy_registry.len()).choose(rng.deref_mut()).unwrap());

            let selected_enemy_pack = &enemy_registry[selected_enemy_pack_index].pack;
            log::info!("randomly selected {:?} enemies", selected_enemy_pack.name());

            commands.insert_resource(selected_enemy_pack_index);
        },
    }
}
