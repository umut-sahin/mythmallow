use crate::{
    prelude::*,
    ui::game_over_menu::{
        constants::*,
        styles,
    },
};


/// Spawns the game over menu.
pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_over_menu_action_input_map: Res<InputMap<GameOverMenuAction>>,
    game_result: Res<GameResult>,
) {
    let button_style = styles::button();
    let button_colors = WidgetColors::button();
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_size = BUTTON_FONT_SIZE;

    let mut widgets = Vec::with_capacity(3);

    let title_text = match *game_result {
        GameResult::Won => {
            let play_again_button = Widget::button(
                &mut commands,
                (GameOverMenuPlayAgainButton, Widget::default().selected(), WidgetSelected::new()),
                &button_style,
                button_colors,
                &button_font,
                button_size,
                "Play Again",
            );
            widgets.push(play_again_button);

            "You won!"
        },
        GameResult::Lost => {
            let retry_button = Widget::button(
                &mut commands,
                (GameOverMenuRetryButton, Widget::default().selected(), WidgetSelected::new()),
                &button_style,
                button_colors,
                &button_font,
                button_size,
                "Retry",
            );
            widgets.push(retry_button);

            "You lost!"
        },
    };

    let return_to_main_menu_button = Widget::button(
        &mut commands,
        (GameOverMenuReturnToMainMenuButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_size,
        "Return to main menu",
    );
    widgets.push(return_to_main_menu_button);

    let quit_to_desktop_button = Widget::button(
        &mut commands,
        (GameOverMenuQuitToDesktopButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_size,
        "Quit to desktop",
    );
    widgets.push(quit_to_desktop_button);

    for i in 0..widgets.len() {
        let up = if i != 0 { widgets[i - 1] } else { widgets[widgets.len() - 1] };
        let current = widgets[i];
        let down = if i != widgets.len() - 1 { widgets[i + 1] } else { widgets[0] };

        commands.entity(current).insert((WidgetUp(up), WidgetDown(down)));
    }

    let title = commands
        .spawn((
            GameOverMenuTitle,
            TextBundle {
                style: styles::title(),
                text: Text::from_section(
                    title_text,
                    TextStyle { font_size: TITLE_FONT_SIZE, ..default() },
                ),
                ..default()
            },
        ))
        .id();

    let mut action_state = ActionState::default();

    let pressed = ActionData { state: ButtonState::Pressed, ..default() };
    action_state.set_action_data(GameOverMenuAction::Select, pressed);

    let mut game_over_menu = commands.spawn((
        GameOverMenu,
        InputManagerBundle::<GameOverMenuAction> {
            action_state,
            input_map: game_over_menu_action_input_map.clone(),
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

    game_over_menu.add_child(title);
    for widget in widgets {
        game_over_menu.add_child(widget);
    }
}

/// Despawns the game over menu.
pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    commands.remove_resource::<GameResult>();
    if let Ok(entity) = game_over_menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Navigates the game over menu using game over menu actions.
pub fn navigation(
    mut commands: Commands,
    mut game_over_menu_query: Query<&ActionState<GameOverMenuAction>, With<GameOverMenu>>,
    mut selected_widget_query: Query<(&WidgetUp, &mut Widget, &WidgetDown), With<WidgetSelected>>,
) {
    let game_over_menu_action_state = match game_over_menu_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let (up_widget, mut selected_widget, down_widget) = match selected_widget_query.get_single_mut()
    {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if game_over_menu_action_state.just_pressed(GameOverMenuAction::Select) {
        selected_widget.clicked = true;
        return;
    }

    let go_up = game_over_menu_action_state.just_pressed(GameOverMenuAction::Up);
    let go_down = game_over_menu_action_state.just_pressed(GameOverMenuAction::Down);

    if (go_up || go_down) && !(go_up && go_down) {
        if go_down {
            commands.entity(down_widget.0).insert(WidgetSelected::new());
        } else {
            commands.entity(up_widget.0).insert(WidgetSelected::new());
        }
    }
}

/// Restarts the game if play again button is clicked.
pub fn play_again_button_interaction(
    mut play_again_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<GameOverMenuPlayAgainButton>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = play_again_button_query.get_single_mut() {
        button.on_click(|| {
            next_game_state.set(GameState::Restart);
        });
    }
}

/// Restarts the game if retry button is clicked.
pub fn retry_button_interaction(
    mut retry_button_query: Query<&mut Widget, (Changed<Widget>, With<GameOverMenuRetryButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = retry_button_query.get_single_mut() {
        button.on_click(|| {
            next_game_state.set(GameState::Restart);
        });
    }
}

/// Returns to main menu if return to main menu button is clicked.
pub fn return_to_main_menu_button_interaction(
    mut return_to_main_menu_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<GameOverMenuReturnToMainMenuButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = return_to_main_menu_button_query.get_single_mut() {
        button.on_click(|| {
            next_app_state.set(AppState::MainMenu);
            next_game_state.set(GameState::None);
        });
    }
}

/// Quits to desktop if quit to desktop button is clicked.
pub fn quit_to_desktop_button_interaction(
    mut quit_to_desktop_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<GameOverMenuQuitToDesktopButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok(mut button) = quit_to_desktop_button_query.get_single_mut() {
        button.on_click(|| {
            app_exit_event_writer.send(AppExit);
        });
    }
}
