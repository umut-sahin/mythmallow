use crate::{
    prelude::*,
    ui::game_mode_selection_screen::{
        constants::*,
        styles,
    },
};


/// Spawns the game mode selection screen.
pub fn spawn_game_mode_selection_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_mode_selection_screen_action_input_map: Res<InputMap<GameModeSelectionScreenAction>>,
    game_mode_registry: Res<GameModeRegistry>,
) {
    if game_mode_registry.is_empty() {
        // TODO: Replace panic with a proper error communicated through the UI.
        panic!("no game modes are available");
    }

    if game_mode_registry.number_of_game_modes() == 1 {
        let selected_game_mode_index = SelectedGameModeIndex(0);
        commands.insert_resource(selected_game_mode_index);
        return;
    }

    let button_style = styles::button();
    let button_colors = WidgetColors::button();
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_size = BUTTON_FONT_SIZE;

    let mut entities = Vec::new();
    let mut first = true;

    for (game_mode_index, game_mode) in game_mode_registry.iter().enumerate() {
        let game_mode_index = SelectedGameModeIndex(game_mode_index);
        let game_mode_button = if first {
            first = false;
            Widget::button(
                &mut commands,
                (
                    Name::new(format!("{} Button", game_mode.name())),
                    GameModeSelectionScreenGameModeButton { game_mode_index },
                    Widget::default().selected(),
                    WidgetSelected::now(),
                ),
                &button_style,
                button_colors,
                &button_font,
                button_size,
                game_mode.name(),
            )
        } else {
            Widget::button(
                &mut commands,
                (
                    Name::new(format!("{} Button", game_mode.name())),
                    GameModeSelectionScreenGameModeButton { game_mode_index },
                    Widget::default(),
                ),
                &button_style,
                button_colors,
                &button_font,
                button_size,
                game_mode.name(),
            )
        };
        entities.push(game_mode_button)
    }

    let back_button = Widget::button(
        &mut commands,
        (Name::new("Back Button"), GameModeSelectionScreenBackButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_size,
        "Back",
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

    action_state.set_action_data(GameModeSelectionScreenAction::Back, pressed.clone());
    action_state.set_action_data(GameModeSelectionScreenAction::Up, pressed.clone());
    action_state.set_action_data(GameModeSelectionScreenAction::Down, pressed.clone());
    action_state.set_action_data(GameModeSelectionScreenAction::Select, pressed);

    let mut game_mode_selection_screen = commands.spawn((
        Name::new("GameMode Selection Screen"),
        GameModeSelectionScreen,
        InputManagerBundle::<GameModeSelectionScreenAction> {
            action_state,
            input_map: game_mode_selection_screen_action_input_map.clone(),
        },
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(ROW_GAP),
                ..default()
            },
            ..default()
        },
    ));

    for entity in entities {
        game_mode_selection_screen.add_child(entity);
    }
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


/// Navigates the game mode selection screen using game mode selection screen actions.
pub fn navigation(
    mut commands: Commands,
    mut game_mode_selection_screen_query: Query<
        &ActionState<GameModeSelectionScreenAction>,
        With<GameModeSelectionScreen>,
    >,
    mut selected_widget_query: Query<(&mut Widget, &WidgetUp, &WidgetDown), With<WidgetSelected>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    let game_mode_selection_screen_action_state =
        match game_mode_selection_screen_query.get_single_mut() {
            Ok(query_result) => query_result,
            Err(_) => return,
        };
    let (mut selected_widget, up_widget, down_widget) = match selected_widget_query.get_single_mut()
    {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if game_mode_selection_screen_action_state.just_pressed(&GameModeSelectionScreenAction::Back) {
        next_app_state.set(AppState::MainMenu);
        return;
    }

    if game_mode_selection_screen_action_state.just_pressed(&GameModeSelectionScreenAction::Select)
    {
        selected_widget.clicked = true;
        return;
    }

    let go_up =
        game_mode_selection_screen_action_state.just_pressed(&GameModeSelectionScreenAction::Up);
    let go_down =
        game_mode_selection_screen_action_state.just_pressed(&GameModeSelectionScreenAction::Down);

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
        (Changed<Widget>, With<GameModeSelectionScreenBackButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(mut button) = back_button_query.get_single_mut() {
        button.on_click(|| {
            next_app_state.set(AppState::MainMenu);
        });
    }
}

/// Selects the game mode.
pub fn game_mode_button_interaction(
    mut commands: Commands,
    mut game_mode_button_query: Query<
        (&mut Widget, &GameModeSelectionScreenGameModeButton),
        Changed<Widget>,
    >,
) {
    for (mut button, metadata) in &mut game_mode_button_query {
        button.on_click(|| {
            commands.insert_resource(metadata.game_mode_index);
        });
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
    game_mode_registry: Res<GameModeRegistry>,
) {
    match &args.start_in_game_mode {
        Some(specified_game_mode_id_and_args) => {
            let specified_game_mode_id = specified_game_mode_id_and_args.split(' ').next().unwrap();
            for (game_mode_index, entry) in game_mode_registry.iter().enumerate() {
                if entry.game_mode.id() == specified_game_mode_id {
                    log::info!("selected manually specified {:?} game mode", entry.game_mode.id());

                    let selected_game_mode_index = SelectedGameModeIndex(game_mode_index);
                    commands.insert_resource(selected_game_mode_index);
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

            let selected_game_mode_index = SelectedGameModeIndex(
                (0..game_mode_registry.len()).choose(rng.deref_mut()).unwrap(),
            );

            let selected_game_mode = &game_mode_registry[selected_game_mode_index];
            log::info!("randomly selected {:?} game mode", selected_game_mode.name());

            commands.insert_resource(selected_game_mode_index);
        },
    }
}
