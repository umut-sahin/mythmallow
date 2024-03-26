use crate::{
    prelude::*,
    ui::main_menu::{
        constants::*,
        styles,
    },
};


/// Spawns the main menu.
pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    main_menu_action_input_map: Res<InputMap<MainMenuAction>>,
) {
    let button_style = styles::button();
    let button_colors = WidgetColors::button();
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_size = BUTTON_FONT_SIZE;

    let play_button = Widget::button(
        &mut commands,
        (
            Name::new("Play Button"),
            MainMenuPlayButton,
            Widget::default().selected(),
            WidgetSelected::now(),
        ),
        &button_style,
        button_colors,
        &button_font,
        button_size,
        "Play",
    );
    let quit_button = Widget::button(
        &mut commands,
        (Name::new("Quit Button"), MainMenuQuitButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_size,
        "Quit",
    );

    let entities = [play_button, quit_button];
    for i in 0..entities.len() {
        let up = if i != 0 { entities[i - 1] } else { entities[entities.len() - 1] };
        let current = entities[i];
        let down = if i != entities.len() - 1 { entities[i + 1] } else { entities[0] };

        commands.entity(current).insert((WidgetUp(up), WidgetDown(down)));
    }

    let mut action_state = ActionState::default();

    let pressed = ActionData { state: ButtonState::Pressed, ..default() };

    action_state.set_action_data(MainMenuAction::Up, pressed.clone());
    action_state.set_action_data(MainMenuAction::Down, pressed.clone());
    action_state.set_action_data(MainMenuAction::Select, pressed);

    let mut main_menu = commands.spawn((
        Name::new("Main Menu"),
        MainMenu,
        InputManagerBundle::<MainMenuAction> {
            action_state,
            input_map: main_menu_action_input_map.clone(),
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
        main_menu.add_child(entity);
    }
}

/// Despawns the main menu.
pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = main_menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Navigates the main menu using main menu actions.
pub fn navigation(
    mut commands: Commands,
    mut main_menu_query: Query<&ActionState<MainMenuAction>, With<MainMenu>>,
    mut selected_widget_query: Query<(&mut Widget, &WidgetUp, &WidgetDown), With<WidgetSelected>>,
) {
    let main_menu_action_state = match main_menu_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let (mut selected_widget, up_widget, down_widget) = match selected_widget_query.get_single_mut()
    {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if main_menu_action_state.just_pressed(&MainMenuAction::Select) {
        selected_widget.clicked = true;
        return;
    }

    let go_up = main_menu_action_state.just_pressed(&MainMenuAction::Up);
    let go_down = main_menu_action_state.just_pressed(&MainMenuAction::Down);

    if (go_up || go_down) && !(go_up && go_down) {
        if go_down {
            commands.entity(down_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(up_widget.0).insert(WidgetSelected::now());
        }
    }
}


/// Transitions to the game mode selection screen.
pub fn play_button_interaction(
    mut play_button_query: Query<&mut Widget, (Changed<Widget>, With<MainMenuPlayButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(mut button) = play_button_query.get_single_mut() {
        button.on_click(|| {
            next_app_state.set(AppState::GameModeSelectionScreen);
        });
    }
}

#[cfg(feature = "native")]
/// Quits the application.
pub fn quit_button_interaction(
    mut quit_button_query: Query<&mut Widget, (Changed<Widget>, With<MainMenuQuitButton>)>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok(mut button) = quit_button_query.get_single_mut() {
        button.on_click(|| {
            app_exit_event_writer.send(AppExit);
        });
    }
}

#[cfg(feature = "wasm")]
/// Quits the application.
pub fn quit_button_interaction(
    mut quit_button_query: Query<&mut Widget, (Changed<Widget>, With<MainMenuQuitButton>)>,
) {
    if let Ok(mut button) = quit_button_query.get_single_mut() {
        button.on_click(|| {
            let window = match web_sys::window() {
                Some(window) => window,
                None => {
                    log::error!("unable to get the window to close");
                    return;
                },
            };
            if let Err(error) = window.close() {
                log::error!("unable to close the window ({:?})", error);
            }
        });
    }
}
