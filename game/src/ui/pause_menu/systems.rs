use crate::{
    prelude::*,
    ui::pause_menu::{
        constants::*,
        localization,
        styles,
    },
};


/// Spawns the pause menu.
pub fn spawn_pause_menu(
    mut commands: Commands,
    mut pause_menu_query: Query<Entity, With<PauseMenu>>,
    mut child_query: Query<(&Parent, &mut Visibility)>,
    asset_server: Res<AssetServer>,
    pause_menu_action_input_map: Res<InputMap<PauseMenuAction>>,
    previously_selected_widget: Option<Res<PreviouslySelectedPauseMenuWidget>>,
    localization: Res<Localization>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single_mut() {
        if let Some(previously_selected_widget) = previously_selected_widget {
            commands.entity(previously_selected_widget.0).insert(WidgetSelected::now());
            commands.remove_resource::<PreviouslySelectedPauseMenuWidget>();
        }
        for (parent, mut visibility) in child_query.iter_mut() {
            if parent.get() == pause_menu_entity {
                *visibility = Visibility::Visible;
            }
        }
        return;
    }

    let button_style = styles::button();
    let button_colors = WidgetColors::button();
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_font_size = BUTTON_FONT_SIZE;

    let resume_button = Widget::button(
        &mut commands,
        (
            Name::new("Resume Button"),
            PauseMenuResumeButton,
            Widget::default().selected(),
            WidgetSelected::now(),
        ),
        &button_style,
        button_colors,
        &button_font,
        button_font_size,
        localization::resume_button(),
        &localization,
    );
    let settings_button = Widget::button(
        &mut commands,
        (Name::new("Settings Button"), PauseMenuSettingsButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_font_size,
        localization::settings_button(),
        &localization,
    );
    let return_to_main_menu_button = Widget::button(
        &mut commands,
        (
            Name::new("Return To Main Menu Button"),
            PauseMenuReturnToMainMenuButton,
            Widget::default(),
        ),
        &button_style,
        button_colors,
        &button_font,
        button_font_size,
        localization::return_to_main_menu_button(),
        &localization,
    );
    let quit_button = Widget::button(
        &mut commands,
        (Name::new("Quit Button"), PauseMenuQuitButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_font_size,
        localization::quit_button(),
        &localization,
    );

    let entities = [resume_button, settings_button, return_to_main_menu_button, quit_button];
    for i in 0..entities.len() {
        let up = if i != 0 { entities[i - 1] } else { entities[entities.len() - 1] };
        let current = entities[i];
        let down = if i != entities.len() - 1 { entities[i + 1] } else { entities[0] };

        commands.entity(current).insert((WidgetUp(up), WidgetDown(down)));
    }

    let pressed = ActionData { state: ButtonState::Pressed, ..default() };
    let mut action_state = ActionState::default();

    action_state.set_action_data(PauseMenuAction::Resume, pressed.clone());
    action_state.set_action_data(PauseMenuAction::Up, pressed.clone());
    action_state.set_action_data(PauseMenuAction::Down, pressed.clone());
    action_state.set_action_data(PauseMenuAction::Select, pressed);

    let mut pause_menu = commands.spawn((
        Name::new("Pause Menu"),
        PauseMenu,
        InputManagerBundle::<PauseMenuAction> {
            action_state,
            input_map: pause_menu_action_input_map.clone(),
        },
        NodeBundle {
            style: styles::root(),
            background_color: BackgroundColor(BACKGROUND_COLOR),
            z_index: ZIndex::Global(1),
            ..default()
        },
    ));

    for entity in entities {
        pause_menu.add_child(entity);
    }
}

/// Despawns the pause menu.
pub fn despawn_pause_menu(
    mut commands: Commands,
    mut pause_menu_query: Query<Entity, With<PauseMenu>>,
    mut child_query: Query<(&Parent, &mut Visibility)>,
    widget_query: Query<Entity, With<WidgetSelected>>,
    app_state: Res<State<AppState>>,
    game_state_stack: Res<GameStateStack>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single_mut() {
        if app_state.get() == &AppState::Game && game_state_stack.contains(&GameState::Paused) {
            for (parent, mut visibility) in child_query.iter_mut() {
                if parent.get() == pause_menu_entity {
                    *visibility = Visibility::Hidden;
                }
            }
            if let Ok(widget) = widget_query.get_single() {
                commands.insert_resource(PreviouslySelectedPauseMenuWidget(widget));
            }
            return;
        }
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}


/// Navigates the pause menu using pause menu actions.
pub fn navigation(
    mut commands: Commands,
    mut pause_menu_query: Query<&ActionState<PauseMenuAction>, With<PauseMenu>>,
    mut selected_widget_query: Query<(&mut Widget, &WidgetUp, &WidgetDown), With<WidgetSelected>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let pause_menu_action_state = match pause_menu_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let (mut selected_widget, up_widget, down_widget) = match selected_widget_query.get_single_mut()
    {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if pause_menu_action_state.just_pressed(&PauseMenuAction::Resume) {
        game_state_stack.pop();
        next_game_state.set(GameState::Transition);
        return;
    }

    if pause_menu_action_state.just_pressed(&PauseMenuAction::Select) {
        selected_widget.clicked = true;
        return;
    }

    let go_up = pause_menu_action_state.just_pressed(&PauseMenuAction::Up);
    let go_down = pause_menu_action_state.just_pressed(&PauseMenuAction::Down);

    if (go_up || go_down) && !(go_up && go_down) {
        if go_down {
            commands.entity(down_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(up_widget.0).insert(WidgetSelected::now());
        }
    }
}


/// Resumes the game.
pub fn resume_button_interaction(
    mut resume_button_query: Query<&mut Widget, (Changed<Widget>, With<PauseMenuResumeButton>)>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = resume_button_query.get_single_mut() {
        button.on_click(|| {
            game_state_stack.pop();
            next_game_state.set(GameState::Transition);
        });
    }
}

/// Opens the settings menu.
pub fn settings_button_interaction(
    mut settings_button_query: Query<&mut Widget, (Changed<Widget>, With<PauseMenuSettingsButton>)>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = settings_button_query.get_single_mut() {
        button.on_click(|| {
            game_state_stack.push(GameState::Settings);
            next_game_state.set(GameState::Transition);
        });
    }
}

/// Returns to the main menu.
pub fn return_to_main_menu_button_interaction(
    mut return_to_main_menu_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<PauseMenuReturnToMainMenuButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(mut button) = return_to_main_menu_button_query.get_single_mut() {
        button.on_click(|| {
            next_app_state.set(AppState::MainMenu);
        });
    }
}

#[cfg(feature = "native")]
/// Quits the application.
pub fn quit_button_interaction(
    mut quit_button_query: Query<&mut Widget, (Changed<Widget>, With<PauseMenuQuitButton>)>,
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
    mut quit_button_query: Query<&mut Widget, (Changed<Widget>, With<PauseMenuQuitButton>)>,
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
