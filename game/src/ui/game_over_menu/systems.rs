use crate::{
    prelude::*,
    ui::game_over_menu::{
        constants::*,
        localization,
        styles,
    },
};


/// Spawns the game over menu.
pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_over_menu_action_input_map: Res<InputMap<GameOverMenuAction>>,
    game_result: Res<GameResult>,
    localization: Res<Localization>,
) {
    let button_style = styles::button();
    let button_colors = WidgetColors::button();
    let button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_font_size = BUTTON_FONT_SIZE;

    let mut widgets = Vec::with_capacity(3);

    let title_text = match *game_result {
        GameResult::Won => {
            let play_again_button = Widget::button(
                &mut commands,
                (
                    Name::new("Play Again Button"),
                    GameOverMenuPlayAgainButton,
                    Widget::default().selected(),
                    WidgetSelected::now(),
                ),
                &button_style,
                button_colors,
                &button_font,
                button_font_size,
                localization::play_again_button(),
                &localization,
            );
            widgets.push(play_again_button);

            localization::won_title()
        },
        GameResult::Lost => {
            let retry_button = Widget::button(
                &mut commands,
                (
                    Name::new("Retry Button"),
                    GameOverMenuRetryButton,
                    Widget::default().selected(),
                    WidgetSelected::now(),
                ),
                &button_style,
                button_colors,
                &button_font,
                button_font_size,
                localization::retry_button(),
                &localization,
            );
            widgets.push(retry_button);

            localization::lost_title()
        },
    };

    let return_to_main_menu_button = Widget::button(
        &mut commands,
        (
            Name::new("Return To Main Menu Button"),
            GameOverMenuReturnToMainMenuButton,
            Widget::default(),
        ),
        &button_style,
        button_colors,
        &button_font,
        button_font_size,
        localization::return_to_main_menu_button(),
        &localization,
    );
    widgets.push(return_to_main_menu_button);

    let quit_button = Widget::button(
        &mut commands,
        (Name::new("Quit Button"), GameOverMenuQuitButton, Widget::default()),
        &button_style,
        button_colors,
        &button_font,
        button_font_size,
        localization::quit_button(),
        &localization,
    );
    widgets.push(quit_button);

    for i in 0..widgets.len() {
        let up = if i != 0 { widgets[i - 1] } else { widgets[widgets.len() - 1] };
        let current = widgets[i];
        let down = if i != widgets.len() - 1 { widgets[i + 1] } else { widgets[0] };

        commands.entity(current).insert((WidgetUp(up), WidgetDown(down)));
    }

    let title = commands
        .spawn((
            Name::new("Title"),
            GameOverMenuTitle,
            TextBundle {
                style: styles::title(),
                text: Text::from_section(
                    title_text.get(&localization),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: TITLE_FONT_SIZE,
                        ..default()
                    },
                ),
                ..default()
            },
            title_text,
        ))
        .id();

    let mut action_state = ActionState::default();

    let pressed = ActionData { state: ButtonState::Pressed, ..default() };

    action_state.set_action_data(GameOverMenuAction::Up, pressed.clone());
    action_state.set_action_data(GameOverMenuAction::Down, pressed.clone());
    action_state.set_action_data(GameOverMenuAction::Select, pressed);

    let mut game_over_menu = commands.spawn((
        Name::new("Game Over Menu"),
        GameOverMenu,
        InputManagerBundle::<GameOverMenuAction> {
            action_state,
            input_map: game_over_menu_action_input_map.clone(),
        },
        NodeBundle { style: styles::root(), ..default() },
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
    mut selected_widget_query: Query<(&mut Widget, &WidgetUp, &WidgetDown), With<WidgetSelected>>,
) {
    let game_over_menu_action_state = match game_over_menu_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let (mut selected_widget, up_widget, down_widget) = match selected_widget_query.get_single_mut()
    {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if game_over_menu_action_state.just_pressed(&GameOverMenuAction::Select) {
        selected_widget.clicked = true;
        return;
    }

    let go_up = game_over_menu_action_state.just_pressed(&GameOverMenuAction::Up);
    let go_down = game_over_menu_action_state.just_pressed(&GameOverMenuAction::Down);

    if (go_up || go_down) && !(go_up && go_down) {
        if go_down {
            commands.entity(down_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(up_widget.0).insert(WidgetSelected::now());
        }
    }
}


/// Restarts the game.
pub fn play_again_button_interaction(
    mut play_again_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<GameOverMenuPlayAgainButton>),
    >,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = play_again_button_query.get_single_mut() {
        button.on_click(|| {
            game_state_stack.transition(GameState::Restart);
            next_game_state.set(GameState::Transition);
        });
    }
}

/// Restarts the game.
pub fn retry_button_interaction(
    mut retry_button_query: Query<&mut Widget, (Changed<Widget>, With<GameOverMenuRetryButton>)>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = retry_button_query.get_single_mut() {
        button.on_click(|| {
            game_state_stack.transition(GameState::Restart);
            next_game_state.set(GameState::Transition);
        });
    }
}

/// Returns to the main menu.
pub fn return_to_main_menu_button_interaction(
    mut return_to_main_menu_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<GameOverMenuReturnToMainMenuButton>),
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
    mut quit_button_query: Query<&mut Widget, (Changed<Widget>, With<GameOverMenuQuitButton>)>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok(mut button) = quit_button_query.get_single_mut() {
        button.on_click(|| {
            app_exit_event_writer.send(AppExit::Success);
        });
    }
}

#[cfg(feature = "wasm")]
/// Quits the application.
pub fn quit_button_interaction(
    mut quit_button_query: Query<&mut Widget, (Changed<Widget>, With<GameOverMenuQuitButton>)>,
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
