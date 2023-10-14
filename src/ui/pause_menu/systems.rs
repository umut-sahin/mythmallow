use crate::{
    prelude::*,
    ui::pause_menu::{
        constants::*,
        styles,
    },
};


/// Spawns the pause menu.
pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pause_menu_action_input_map: Res<InputMap<PauseMenuAction>>,
) {
    let button_style = styles::button();
    let button_colors = WidgetColors::button();
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_size = BUTTON_FONT_SIZE;

    let resume_button = Widget::button(
        &mut commands,
        (PauseMenuResumeButton, Widget::default().selected(), WidgetSelected::new()),
        &button_style,
        button_colors,
        &button_font,
        button_size,
        "Resume",
    );
    let return_to_main_menu_button = Widget::button(
        &mut commands,
        (PauseMenuReturnToMainMenuButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_size,
        "Return to main menu",
    );
    let quit_to_desktop_button = Widget::button(
        &mut commands,
        (PauseMenuQuitToDesktopButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_size,
        "Quit to desktop",
    );

    let entities = [resume_button, return_to_main_menu_button, quit_to_desktop_button];
    for i in 0..entities.len() {
        let up = if i != 0 { entities[i - 1] } else { entities[entities.len() - 1] };
        let current = entities[i];
        let down = if i != entities.len() - 1 { entities[i + 1] } else { entities[0] };

        commands.entity(current).insert((WidgetUp(up), WidgetDown(down)));
    }

    let pressed = ActionData { state: ButtonState::Pressed, ..default() };
    let mut action_state = ActionState::default();

    action_state.set_action_data(PauseMenuAction::Resume, pressed.clone());
    action_state.set_action_data(PauseMenuAction::Select, pressed);

    let mut pause_menu = commands.spawn((
        PauseMenu,
        InputManagerBundle::<PauseMenuAction> {
            action_state,
            input_map: pause_menu_action_input_map.clone(),
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
        pause_menu.add_child(entity);
    }
}

/// Despawns the pause menu.
pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(entity) = pause_menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Navigates the pause menu using pause menu actions.
pub fn navigation(
    mut commands: Commands,
    mut pause_menu_query: Query<&ActionState<PauseMenuAction>, With<PauseMenu>>,
    mut selected_widget_query: Query<(&WidgetUp, &mut Widget, &WidgetDown), With<WidgetSelected>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let pause_menu_action_state = match pause_menu_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let (up_widget, mut selected_widget, down_widget) = match selected_widget_query.get_single_mut()
    {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if pause_menu_action_state.just_pressed(PauseMenuAction::Resume) {
        next_game_state.set(GameState::Playing);
        return;
    }

    if pause_menu_action_state.just_pressed(PauseMenuAction::Select) {
        selected_widget.clicked = true;
        return;
    }

    let go_up = pause_menu_action_state.just_pressed(PauseMenuAction::Up);
    let go_down = pause_menu_action_state.just_pressed(PauseMenuAction::Down);

    if (go_up || go_down) && !(go_up && go_down) {
        if go_down {
            commands.entity(down_widget.0).insert(WidgetSelected::new());
        } else {
            commands.entity(up_widget.0).insert(WidgetSelected::new());
        }
    }
}

/// Resumes the game if resume button is clicked.
pub fn resume_button_interaction(
    mut resume_button_query: Query<&mut Widget, (Changed<Widget>, With<PauseMenuResumeButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = resume_button_query.get_single_mut() {
        button.on_click(|| next_game_state.set(GameState::Playing));
    }
}

/// Returns to main menu if return to main menu button is clicked.
pub fn return_to_main_menu_button_interaction(
    mut return_to_main_menu_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<PauseMenuReturnToMainMenuButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = return_to_main_menu_button_query.get_single_mut() {
        button.on_click(|| {
            next_app_state.set(AppState::MainMenu);
            next_game_state.set(GameState::Playing);
        });
    }
}

/// Quits to desktop if quit to desktop button is clicked.
pub fn quit_to_desktop_button_interaction(
    mut quit_to_desktop_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<PauseMenuQuitToDesktopButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok(mut button) = quit_to_desktop_button_query.get_single_mut() {
        button.on_click(|| app_exit_event_writer.send(AppExit));
    }
}
