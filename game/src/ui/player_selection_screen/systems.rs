use crate::{
    prelude::*,
    ui::player_selection_screen::{
        constants::*,
        localization,
        styles,
    },
};


/// Spawns the player selection screen.
pub fn spawn_player_selection_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_selection_screen_action_input_map: Res<InputMap<PlayerSelectionScreenAction>>,
    player_registry: Res<PlayerRegistry>,
    localization: Res<Localization>,
) {
    let button_style = styles::button();
    let button_colors = WidgetColors::button();
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_font_size = BUTTON_FONT_SIZE;

    let mut entities = Vec::new();
    let mut first = true;

    for (mythology_index, entry) in player_registry.iter().enumerate() {
        // TODO: Group player buttons by mythology.
        for (player_index, player) in entry.players.iter().enumerate() {
            let mythology_index = SelectedMythologyIndex(mythology_index);
            let player_index = SelectedPlayerIndex(player_index);

            let player_button = if first {
                first = false;
                Widget::button(
                    &mut commands,
                    (
                        Name::new(format!("Player Button [{}]", player.id())),
                        PlayerSelectionScreenPlayerButton { mythology_index, player_index },
                        Widget::default().selected(),
                        WidgetSelected::now(),
                    ),
                    &button_style,
                    button_colors,
                    &button_font,
                    button_font_size,
                    player.name(),
                    &localization,
                )
            } else {
                Widget::button(
                    &mut commands,
                    (
                        Name::new(format!("Player Button [{}]", player.id())),
                        PlayerSelectionScreenPlayerButton { mythology_index, player_index },
                        Widget::default(),
                    ),
                    &button_style,
                    button_colors,
                    &button_font,
                    button_font_size,
                    player.name(),
                    &localization,
                )
            };
            entities.push(player_button)
        }
    }

    let back_button = Widget::button(
        &mut commands,
        (Name::new("Back Button"), PlayerSelectionScreenBackButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_font_size,
        localization::back_button(),
        &localization,
    );
    entities.push(back_button);

    for i in 0..entities.len() {
        let up = if i != 0 { entities[i - 1] } else { entities[entities.len() - 1] };
        let current = entities[i];
        let down = if i != entities.len() - 1 { entities[i + 1] } else { entities[0] };

        commands.entity(current).insert((WidgetUp(up), WidgetDown(down)));
    }

    let pressed = ActionData { state: ButtonState::Pressed, ..default() };
    let mut action_state = ActionState::default();

    action_state.set_action_data(PlayerSelectionScreenAction::Back, pressed.clone());
    action_state.set_action_data(PlayerSelectionScreenAction::Up, pressed.clone());
    action_state.set_action_data(PlayerSelectionScreenAction::Down, pressed.clone());
    action_state.set_action_data(PlayerSelectionScreenAction::Select, pressed);

    let mut player_selection_screen = commands.spawn((
        Name::new("Player Selection Screen"),
        PlayerSelectionScreen,
        InputManagerBundle::<PlayerSelectionScreenAction> {
            action_state,
            input_map: player_selection_screen_action_input_map.clone(),
        },
        NodeBundle { style: styles::root(), ..default() },
    ));

    for entity in entities {
        player_selection_screen.add_child(entity);
    }
}

/// Despawns the player selection screen.
pub fn despawn_player_selection_screen(
    mut commands: Commands,
    player_selection_screen_query: Query<Entity, With<PlayerSelectionScreen>>,
) {
    if let Ok(entity) = player_selection_screen_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Navigates the player selection screen using player selection screen actions.
pub fn navigation(
    mut commands: Commands,
    mut player_selection_screen_query: Query<
        &ActionState<PlayerSelectionScreenAction>,
        With<PlayerSelectionScreen>,
    >,
    mut selected_widget_query: Query<(&mut Widget, &WidgetUp, &WidgetDown), With<WidgetSelected>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    let player_selection_screen_action_state = match player_selection_screen_query.get_single_mut()
    {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let (mut selected_widget, up_widget, down_widget) = match selected_widget_query.get_single_mut()
    {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if player_selection_screen_action_state.just_pressed(&PlayerSelectionScreenAction::Back) {
        next_app_state.set(AppState::MainMenu);
        return;
    }

    if player_selection_screen_action_state.just_pressed(&PlayerSelectionScreenAction::Select) {
        selected_widget.clicked = true;
        return;
    }

    let go_up = player_selection_screen_action_state.just_pressed(&PlayerSelectionScreenAction::Up);
    let go_down =
        player_selection_screen_action_state.just_pressed(&PlayerSelectionScreenAction::Down);

    if (go_up || go_down) && !(go_up && go_down) {
        if go_down {
            commands.entity(down_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(up_widget.0).insert(WidgetSelected::now());
        }
    }
}


/// Returns to the main menu.
pub fn back_button_interaction(
    mut back_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<PlayerSelectionScreenBackButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(mut button) = back_button_query.get_single_mut() {
        button.on_click(|| {
            next_app_state.set(AppState::MainMenu);
        });
    }
}

/// Selects the player.
pub fn player_button_interaction(
    mut commands: Commands,
    mut player_button_query: Query<
        (&mut Widget, &PlayerSelectionScreenPlayerButton),
        Changed<Widget>,
    >,
) {
    for (mut button, metadata) in &mut player_button_query {
        button.on_click(|| {
            commands.insert_resource(metadata.mythology_index);
            commands.insert_resource(metadata.player_index);
        });
    }
}


/// Transitions to the enemy selection screen.
pub fn player_selected(mut next_app_state: ResMut<NextState<AppState>>) {
    next_app_state.set(AppState::EnemySelectionScreen);
}


/// Selects the player randomly or from the arguments of the application.
pub fn select_player_when_starting_in_game(
    mut commands: Commands,
    args: ResMut<Args>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    player_registry: Res<PlayerRegistry>,
) {
    match &args.start_in_game_player {
        Some(specified_player_id) => {
            if let Some((mythology_index, player_index)) =
                player_registry.find_player(specified_player_id)
            {
                let selected_mythology = &player_registry[mythology_index];
                let selected_player = &selected_mythology[player_index];

                log::info!(
                    "selected manually specified {:?} first found in {:?} mythology \
                    as the player",
                    selected_player.id(),
                    selected_mythology.id()
                );

                commands.insert_resource(mythology_index);
                commands.insert_resource(player_index);
            } else {
                log::error!(
                    "couldn't select \
                    manually specified {} as the player \
                    as it isn't registered to any mythologies",
                    specified_player_id,
                );
            }
        },
        None => {
            if player_registry.is_empty() {
                log::error!("couldn't select the player randomly as no players are registered");
                return;
            }

            let number_of_mythologies = player_registry.len();
            let mythology_index =
                SelectedMythologyIndex((0..number_of_mythologies).choose(rng.deref_mut()).unwrap());

            let number_of_players_in_mythology = player_registry[mythology_index].players.len();
            let player_index = SelectedPlayerIndex(
                (0..number_of_players_in_mythology).choose(rng.deref_mut()).unwrap(),
            );

            let selected_mythology = player_registry[mythology_index].clone();
            let selected_player = player_registry[mythology_index][player_index].clone();

            log::info!(
                "randomly selected {:?} from {:?} mythology as the player",
                selected_player.id(),
                selected_mythology.id()
            );

            commands.insert_resource(mythology_index);
            commands.insert_resource(player_index);
        },
    }
}
