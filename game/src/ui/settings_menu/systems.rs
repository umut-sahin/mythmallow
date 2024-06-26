use crate::{
    prelude::*,
    ui::settings_menu::{
        constants::*,
        localization,
        styles,
    },
};


/// Spawns the settings menu.
pub fn spawn_settings_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings_menu_action_input_map: Res<InputMap<SettingsMenuAction>>,
    localization: Res<Localization>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let language_setting_previous_button = {
        let previous = localization::language_setting_previous_button();
        Widget::button(
            &mut commands,
            (
                Name::new("Previous Button"),
                SettingsMenuLanguageSettingPreviousButton,
                Widget::default(),
            ),
            &styles::language_setting_changer(),
            WidgetColors::button(),
            &font,
            LANGUAGE_SETTING_CHANGER_TEXT_FONT_SIZE,
            previous,
            &localization,
        )
    };
    let language_setting_name = {
        let name = localization::language_setting_name();
        commands
            .spawn((
                Name::new("Name"),
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            name.get(&localization),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: LANGUAGE_SETTING_NAME_TEXT_FONT_SIZE,
                                color: LANGUAGE_SETTING_NAME_TEXT_COLOR,
                            },
                        )],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    style: styles::language_setting_name(),
                    ..default()
                },
                name,
            ))
            .id()
    };
    let language_setting_value = {
        let value = localization::language_setting_value();
        commands
            .spawn((
                Name::new("Value"),
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            value.get(&localization),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: LANGUAGE_SETTING_VALUE_TEXT_FONT_SIZE,
                                color: LANGUAGE_SETTING_VALUE_TEXT_COLOR,
                            },
                        )],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    style: styles::language_setting_name(),
                    ..default()
                },
                value,
            ))
            .id()
    };
    let language_setting_next_button = {
        let next = localization::language_setting_next_button();
        Widget::button(
            &mut commands,
            (Name::new("Next Button"), SettingsMenuLanguageSettingNextButton, Widget::default()),
            &styles::language_setting_changer(),
            WidgetColors::button(),
            &font,
            LANGUAGE_SETTING_CHANGER_TEXT_FONT_SIZE,
            next,
            &localization,
        )
    };

    let language_setting = commands
        .spawn((
            Name::new("Language Setting"),
            NodeBundle { style: styles::language_setting_container(), ..default() },
        ))
        .add_child(language_setting_previous_button)
        .add_child(language_setting_name)
        .add_child(language_setting_value)
        .add_child(language_setting_next_button)
        .id();

    let back_button = Widget::button(
        &mut commands,
        (
            Name::new("Back Button"),
            SettingsMenuBackButton,
            Widget::default().selected(),
            WidgetSelected::now(),
        ),
        &styles::back_button(),
        WidgetColors::button(),
        &font,
        BACK_BUTTON_FONT_SIZE,
        localization::back_button(),
        &localization,
    );

    let language_setting_widgets = [language_setting_previous_button, language_setting_next_button];
    let footer_widgets = [back_button];

    let widgets = [language_setting_widgets.as_slice(), footer_widgets.as_slice()];
    for i in 0..widgets.len() {
        let mut upper_row_index = if i != 0 { i - 1 } else { i };
        if upper_row_index != 0 {
            while widgets[upper_row_index].is_empty() {
                upper_row_index -= 1;
                if upper_row_index == 0 {
                    break;
                }
            }
        }
        if widgets[upper_row_index].is_empty() {
            upper_row_index = i;
        }

        let mut lower_row_index = if i != widgets.len() - 1 { i + 1 } else { i };
        if lower_row_index != widgets.len() - 1 {
            while widgets[lower_row_index].is_empty() {
                lower_row_index += 1;
                if lower_row_index == widgets.len() - 1 {
                    break;
                }
            }
        }
        if widgets[lower_row_index].is_empty() {
            lower_row_index = i;
        }

        let row = widgets[i];
        for j in 0..row.len() {
            let left_widget = if j != 0 { row[j - 1] } else { row[j] };
            let current_widget = row[j];
            let right_widget = if j != row.len() - 1 { row[j + 1] } else { row[j] };

            let up_widget = if upper_row_index == i {
                current_widget
            } else if i == widgets.len() - 1 {
                *widgets[upper_row_index].last().unwrap()
            } else {
                *widgets[upper_row_index].get(j).unwrap_or(widgets[upper_row_index].last().unwrap())
            };
            let down_widget = if lower_row_index == i {
                current_widget
            } else {
                *widgets[lower_row_index].get(j).unwrap_or(widgets[lower_row_index].last().unwrap())
            };

            commands.entity(current_widget).insert((
                WidgetUp(up_widget),
                WidgetDown(down_widget),
                WidgetLeft(left_widget),
                WidgetRight(right_widget),
            ));
        }
    }

    let mut action_state = ActionState::default();

    let pressed = ActionData { state: ButtonState::Pressed, ..default() };

    action_state.set_action_data(SettingsMenuAction::Back, pressed.clone());
    action_state.set_action_data(SettingsMenuAction::Up, pressed.clone());
    action_state.set_action_data(SettingsMenuAction::Down, pressed.clone());
    action_state.set_action_data(SettingsMenuAction::Left, pressed.clone());
    action_state.set_action_data(SettingsMenuAction::Right, pressed.clone());
    action_state.set_action_data(SettingsMenuAction::Select, pressed);

    let mut settings_menu = commands.spawn((
        Name::new("Settings Menu"),
        SettingsMenu,
        InputManagerBundle::<SettingsMenuAction> {
            action_state,
            input_map: settings_menu_action_input_map.clone(),
        },
        NodeBundle {
            style: styles::root(),
            background_color: BackgroundColor(BACKGROUND_COLOR),
            z_index: ZIndex::Global(2),
            ..default()
        },
    ));

    settings_menu.add_child(language_setting);
    settings_menu.add_child(back_button);
}

/// Despawns the settings menu.
pub fn despawn_settings_menu(
    mut commands: Commands,
    settings_menu_query: Query<Entity, With<SettingsMenu>>,
) {
    if let Ok(entity) = settings_menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Navigates the settings menu using settings menu actions.
pub fn navigation(
    mut commands: Commands,
    mut settings_menu_query: Query<&ActionState<SettingsMenuAction>, With<SettingsMenu>>,
    mut selected_widget_query: Query<
        (&mut Widget, &WidgetUp, &WidgetDown, &WidgetLeft, &WidgetRight),
        With<WidgetSelected>,
    >,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_state_stack: ResMut<GameStateStack>,
) {
    let settings_menu_action_state = match settings_menu_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let (mut selected_widget, up_widget, down_widget, left_widget, right_widget) =
        match selected_widget_query.get_single_mut() {
            Ok(query_result) => query_result,
            Err(_) => return,
        };

    if settings_menu_action_state.just_pressed(&SettingsMenuAction::Back) {
        if app_state.get() == &AppState::SettingsMenu {
            next_app_state.set(AppState::MainMenu);
        } else {
            assert_eq!(app_state.get(), &AppState::Game);
            game_state_stack.pop();
            next_game_state.set(GameState::Transition);
        }
        return;
    }

    if settings_menu_action_state.just_pressed(&SettingsMenuAction::Select) {
        selected_widget.clicked = true;
        return;
    }

    let go_up = settings_menu_action_state.just_pressed(&SettingsMenuAction::Up);
    let go_down = settings_menu_action_state.just_pressed(&SettingsMenuAction::Down);

    if (go_up || go_down) && !(go_up && go_down) {
        if go_down {
            commands.entity(down_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(up_widget.0).insert(WidgetSelected::now());
        }
    }

    let go_left = settings_menu_action_state.just_pressed(&SettingsMenuAction::Left);
    let go_right = settings_menu_action_state.just_pressed(&SettingsMenuAction::Right);

    if (go_left || go_right) && !(go_left && go_right) {
        if go_right {
            commands.entity(right_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(left_widget.0).insert(WidgetSelected::now());
        }
    }
}


/// Sets the locale to the previous supported locale.
pub fn language_setting_previous_button_interaction(
    mut commands: Commands,
    mut language_setting_previous_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<SettingsMenuLanguageSettingPreviousButton>),
    >,
    supported_locales: Res<SupportedLocales>,
    locale: Res<Locale>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok(mut button) = language_setting_previous_button_query.get_single_mut() {
        button.on_click(|| {
            let position = supported_locales
                .iter()
                .find_position(|supported_locale| *supported_locale == &locale.requested)
                .map(|(position, _)| position)
                .unwrap_or(0);

            let mut new_position = position as isize - 1;
            if new_position < 0 {
                new_position = supported_locales.len() as isize - 1;
            }

            commands.run_system_with_input(
                registered_systems.configuration.set_locale,
                supported_locales[new_position as usize].clone(),
            )
        });
    }
}

/// Sets the locale to the next supported locale.
pub fn language_setting_next_button_interaction(
    mut commands: Commands,
    mut language_setting_next_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<SettingsMenuLanguageSettingNextButton>),
    >,
    supported_locales: Res<SupportedLocales>,
    locale: Res<Locale>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok(mut button) = language_setting_next_button_query.get_single_mut() {
        button.on_click(|| {
            let position = supported_locales
                .iter()
                .find_position(|supported_locale| *supported_locale == &locale.requested)
                .map(|(position, _)| position)
                .unwrap_or(0);

            let mut new_position = position + 1;
            if new_position >= supported_locales.len() {
                new_position = 0;
            }

            commands.run_system_with_input(
                registered_systems.configuration.set_locale,
                supported_locales[new_position].clone(),
            )
        });
    }
}

/// Returns to the main menu.
pub fn back_button_interaction(
    mut back_button_query: Query<&mut Widget, (Changed<Widget>, With<SettingsMenuBackButton>)>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_state_stack: ResMut<GameStateStack>,
) {
    if let Ok(mut button) = back_button_query.get_single_mut() {
        button.on_click(|| {
            if app_state.get() == &AppState::SettingsMenu {
                next_app_state.set(AppState::MainMenu);
            } else {
                assert_eq!(app_state.get(), &AppState::Game);
                game_state_stack.pop();
                next_game_state.set(GameState::Transition);
            }
        });
    }
}
