use crate::{
    prelude::*,
    ui::level_up_screen::{
        commands::*,
        constants::*,
        localization,
        styles,
    },
};


/// Applies the level up screen console commands.
pub fn apply_level_up_screen_command(
    mut commands: Commands,
    mut level_up_screen_configuration: ResMut<LevelUpScreenConfiguration>,
    level_up_screen_state: Option<ResMut<LevelUpScreenState>>,
    level_up_screen_reason: Option<ResMut<LevelUpScreenReason>>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
    registered_systems: Res<RegisteredSystems>,
    perk_registry: Res<PerkRegistry>,
    mut command: ConsoleCommand<LevelUpScreenCommand>,
) {
    if let Some(Ok(LevelUpScreenCommand { subcommand })) = command.take() {
        if *app_state.get() != AppState::Game {
            reply!(command, "Not available outside the game.");
            reply!(command, "");
            return;
        }

        match subcommand {
            LevelUpScreenCommands::Open => {
                match game_state.get() {
                    GameState::Playing => {
                        log::info!("opening the level up screen");
                        commands.insert_resource(LevelUpScreenReason::Cheating);
                        game_state_stack.push(GameState::LevelUpScreen);
                        next_game_state.set(GameState::Transition);
                        reply!(command, "Opened.");
                    },
                    GameState::LevelUpScreen => {
                        reply!(command, "Already opened.",);
                    },
                    GameState::Market => {
                        reply!(command, "Not available in the market.");
                    },
                    GameState::Paused => {
                        reply!(command, "Not available in the pause menu.");
                    },
                    GameState::Settings => {
                        reply!(command, "Not available in the settings menu.");
                    },
                    _ => {
                        reply!(command, "How did you time this, seriously?");
                    },
                }
            },
            LevelUpScreenCommands::Close => {
                match game_state.get() {
                    GameState::LevelUpScreen => {
                        log::info!("closing the level up screen");
                        commands.remove_resource::<LevelUpScreenState>();
                        game_state_stack.pop();
                        next_game_state.set(GameState::Transition);
                        reply!(command, "Closed.");
                    },
                    GameState::Playing => {
                        reply!(command, "Already closed.");
                    },
                    GameState::Market => {
                        reply!(command, "Not available in the market.");
                    },
                    GameState::Paused => {
                        reply!(command, "Not available in the pause menu.");
                    },
                    GameState::Settings => {
                        reply!(command, "Not available in the settings menu.");
                    },
                    _ => {
                        reply!(command, "How did you time this, seriously?");
                    },
                }
            },
            LevelUpScreenCommands::Show => {
                let level_up_screen_state = match level_up_screen_state {
                    Some(resource) => resource,
                    None => {
                        reply!(command, "Not available outside the level up screen.");
                        reply!(command, "");
                        return;
                    },
                };

                for (index, id) in level_up_screen_state.offered_perk_ids.iter().enumerate() {
                    let position = NonZeroUsize::new(index + 1).unwrap();
                    reply!(command, "{}) {}", position, id);
                }
            },
            LevelUpScreenCommands::Select { position } => {
                let level_up_screen_state = match level_up_screen_state {
                    Some(resource) if level_up_screen_reason.is_some() => resource,
                    _ => {
                        reply!(command, "Not available outside the level up screen.");
                        reply!(command, "");
                        return;
                    },
                };
                let mut level_up_screen_reason = level_up_screen_reason.unwrap();

                let index = position.get() - 1;
                if level_up_screen_state.offered_perk_ids.len() <= index {
                    reply!(command, "Failed to select perk {} as it doesn't exist.", position);
                    reply!(command, "");
                    return;
                }

                let selected_perk_id = &level_up_screen_state.offered_perk_ids[index];
                let selected_perk = match perk_registry.find_perk_by_id(selected_perk_id) {
                    Some(perk) => perk,
                    None => {
                        reply!(
                            command,
                            "Failed to select perk {:?} as it not registered.",
                            position,
                        );
                        reply!(command, "");
                        return;
                    },
                };

                let obtain_lose_perk_reason = match level_up_screen_reason.deref_mut() {
                    LevelUpScreenReason::LevelingUp { to } => {
                        let result = ObtainLosePerkReason::LevelingUp { to: *to };
                        to.0 = to.0.checked_add(1).unwrap_or(NonZeroU16::MAX);
                        result
                    },
                    LevelUpScreenReason::Cheating => ObtainLosePerkReason::Cheating,
                };

                commands.remove_resource::<LevelUpScreenState>();
                commands.run_system_with_input(
                    registered_systems.perk.obtain_perk,
                    (selected_perk.deref().clone(), obtain_lose_perk_reason),
                );

                game_state_stack.pop();
                next_game_state.set(GameState::Transition);

                reply!(command, "Selected.");
            },
            LevelUpScreenCommands::Reroll => {
                commands.run_system(registered_systems.level_up_screen.reroll_perks);
                reply!(command, "Rerolled.");
            },
            LevelUpScreenCommands::Offer { position, perk } => {
                let mut level_up_screen_state = match level_up_screen_state {
                    Some(resource) => resource,
                    None => {
                        reply!(command, "Not available outside the level up screen.");
                        reply!(command, "");
                        return;
                    },
                };

                let index = position.get() - 1;
                if level_up_screen_state.offered_perk_ids.len() <= index {
                    reply!(command, "Failed to offer perk {} as it doesn't exist.", position);
                    reply!(command, "");
                    return;
                }

                if perk_registry.find_perk_by_id(&perk).is_none() {
                    reply!(command, "Failed to offer {:?} as it doesn't exist.", perk);
                    reply!(command, "");
                    return;
                }

                log::info!("offering {:?} as perk {} in the level up screen", perk, position);
                level_up_screen_state.offered_perk_ids[index] = perk;
                reply!(command, "Done.");
            },
            LevelUpScreenCommands::NumberOfPerks { subcommand } => {
                match subcommand {
                    NumberOfPerksCommands::Show => {
                        reply!(command, "{}", level_up_screen_configuration.number_of_perks);
                    },
                    NumberOfPerksCommands::Set { number_of_perks } => {
                        log::info!(
                            "setting the number of perks in the level up screen to {}",
                            number_of_perks,
                        );
                        level_up_screen_configuration.number_of_perks = number_of_perks;
                        reply!(command, "Done.");
                    },
                }
            },
            LevelUpScreenCommands::RerollCost { subcommand } => {
                match subcommand {
                    RerollCostCommands::Show => {
                        reply!(command, "{}", level_up_screen_configuration.reroll_cost());
                    },
                    RerollCostCommands::Model { subcommand } => {
                        match subcommand {
                            RerollCostModelCommands::Show => {
                                reply!(command, "{}", level_up_screen_configuration.reroll_cost);
                            },
                            RerollCostModelCommands::Set { subcommand } => {
                                match subcommand {
                                    RerollCostModelSetCommands::Constant { cost } => {
                                        level_up_screen_configuration.reroll_cost =
                                            LevelUpScreenRerollCost::constant(Balance(cost));
                                    },
                                    RerollCostModelSetCommands::Linear {
                                        base,
                                        increase,
                                        step,
                                        max,
                                    } => {
                                        level_up_screen_configuration.reroll_cost =
                                            LevelUpScreenRerollCost::linear(
                                                Balance(base),
                                                Balance(increase),
                                                max.map(Balance),
                                            );
                                        if let Some(step) = step {
                                            level_up_screen_configuration
                                                .reroll_cost
                                                .set_step(step);
                                        }
                                    },
                                    RerollCostModelSetCommands::Exponential {
                                        base,
                                        factor,
                                        step,
                                        max,
                                    } => {
                                        if factor < 1.00 {
                                            reply!(command, "Factor cannot be smaller than 1.00.");
                                            reply!(command, "");
                                            return;
                                        }
                                        level_up_screen_configuration.reroll_cost =
                                            LevelUpScreenRerollCost::exponential(
                                                Balance(base),
                                                factor,
                                                max.map(Balance),
                                            );
                                        if let Some(step) = step {
                                            level_up_screen_configuration
                                                .reroll_cost
                                                .set_step(step);
                                        }
                                    },
                                }
                                log::info!(
                                    "setting the reroll cost model of the level up screen to {}",
                                    level_up_screen_configuration.reroll_cost,
                                );
                                reply!(command, "Done.");
                            },
                        }
                    },
                }
            },
        }
        reply!(command, "");
    }
}


/// Spawns the enemy selection screen.
pub fn spawn_level_up_screen(
    mut commands: Commands,
    level_up_screen_query: Query<&mut Visibility, With<LevelUpScreen>>,
    asset_server: Res<AssetServer>,
    balance: Res<Balance>,
    level_up_screen_action_input_map: Res<InputMap<LevelUpScreenAction>>,
    level_up_screen_configuration: Res<LevelUpScreenConfiguration>,
    level_up_screen_state: Option<ResMut<LevelUpScreenState>>,
    registered_systems: Res<RegisteredSystems>,
    previously_selected_level_up_screen_widget: Option<Res<PreviouslySelectedLevelUpScreenWidget>>,
    localization: Res<Localization>,
) {
    if !level_up_screen_query.is_empty() {
        if let Some(previously_selected_widget) = previously_selected_level_up_screen_widget {
            commands.entity(previously_selected_widget.0).insert(WidgetSelected::now());
            commands.remove_resource::<PreviouslySelectedLevelUpScreenWidget>();
        }
        return;
    }

    if level_up_screen_state.is_none() {
        commands.run_system(registered_systems.level_up_screen.reroll_perks);
    }

    // Create level up action state.
    let mut action_state = ActionState::default();
    {
        let pressed = ActionData { state: ButtonState::Pressed, ..default() };

        action_state.set_action_data(LevelUpScreenAction::Pause, pressed.clone());
        action_state.set_action_data(LevelUpScreenAction::Up, pressed.clone());
        action_state.set_action_data(LevelUpScreenAction::Down, pressed.clone());
        action_state.set_action_data(LevelUpScreenAction::Left, pressed.clone());
        action_state.set_action_data(LevelUpScreenAction::Right, pressed.clone());
        action_state.set_action_data(LevelUpScreenAction::Select, pressed);
    }

    // Root.
    let level_up_screen = {
        commands
            .spawn((
                Name::new("Level Up Screen"),
                LevelUpScreen,
                InputManagerBundle::<LevelUpScreenAction> {
                    action_state,
                    input_map: level_up_screen_action_input_map.clone(),
                },
                NodeBundle {
                    style: styles::level_up_screen(),
                    background_color: BackgroundColor(BACKGROUND_COLOR),
                    ..default()
                },
            ))
            .id()
    };

    // Childrens and widgets.
    let mut level_up_screen_children = Vec::new();
    let mut level_up_screen_widgets = LevelUpScreenWidgets::default();

    // Items.
    {
        let items_container = commands
            .spawn((
                Name::new("Perks"),
                LevelUpScreenPerksContainer,
                NodeBundle { style: styles::perks_container(), ..default() },
            ))
            .id();

        level_up_screen_children.push(items_container);
    }

    // Footer container.
    {
        let footer_container_colors = WidgetColors::container();

        let footer_container = commands
            .spawn((
                Name::new("Footer"),
                LevelUpScreenFooterContainer,
                NodeBundle {
                    style: styles::footer_container(),
                    background_color: footer_container_colors.normal.into(),
                    ..default()
                },
                footer_container_colors,
            ))
            .id();

        level_up_screen_children.push(footer_container);

        {
            // Balance Container.
            let balance_container = {
                let balance_container_style = styles::balance_container();
                let balance_container_colors = WidgetColors::container();

                let balance_container = Widget::container(
                    &mut commands,
                    (Name::new("Balance"), LevelUpScreenBalanceContainer, Widget::default()),
                    &balance_container_style,
                    balance_container_colors,
                );

                let text_style = styles::balance_text();
                let text_font = asset_server.load("fonts/FiraSans-Bold.ttf");
                let text_size = BALANCE_TEXT_FONT_SIZE;

                let balance_text = commands
                    .spawn((
                        Name::new("Text"),
                        LevelUpScreenBalanceText,
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("{:.0}", *balance),
                                    TextStyle {
                                        font: text_font.clone(),
                                        font_size: text_size,
                                        color: balance_container_colors.text,
                                    },
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            style: text_style,
                            ..default()
                        },
                    ))
                    .id();

                commands.entity(balance_container).add_child(balance_text);

                balance_container
            };

            // Reroll button.
            let reroll_button = {
                let reroll_cost = level_up_screen_configuration.reroll_cost();

                let reroll_button_style = styles::reroll_button();
                let reroll_button_colors = WidgetColors::button();
                let reroll_button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
                let reroll_button_size = REROLL_BUTTON_FONT_SIZE;

                let reroll_button = Widget::button(
                    &mut commands,
                    (
                        Name::new("Reroll Button"),
                        LevelUpScreenRerollButton { cost: reroll_cost },
                        Widget::default().selected(),
                        WidgetSelected::now(),
                    ),
                    &reroll_button_style,
                    reroll_button_colors,
                    &reroll_button_font,
                    reroll_button_size,
                    localization::reroll_button(reroll_cost),
                    &localization,
                );

                if *balance < reroll_cost {
                    commands.entity(reroll_button).insert(WidgetDisabled);
                }

                reroll_button
            };

            commands.entity(footer_container).add_child(balance_container).add_child(reroll_button);
            level_up_screen_widgets[1] = vec![balance_container, reroll_button];
        }
    }

    // Add children.
    let mut level_up_screen_entity = commands.entity(level_up_screen);
    for child in level_up_screen_children {
        level_up_screen_entity.add_child(child);
    }

    // Insert widgets.
    commands.insert_resource(level_up_screen_widgets);
}

/// Despawns the level up screen.
pub fn despawn_level_up_screen(
    mut commands: Commands,
    level_up_screen_query: Query<Entity, With<LevelUpScreen>>,
    widget_query: Query<Entity, With<WidgetSelected>>,
    game_state_stack: Res<GameStateStack>,
    app_state: Res<State<AppState>>,
) {
    if app_state.get() == &AppState::Game {
        if let Some(new_state) = game_state_stack.last() {
            if new_state != &GameState::LevelUpScreen
                && game_state_stack.contains(&GameState::LevelUpScreen)
            {
                if let Ok(widget) = widget_query.get_single() {
                    commands.insert_resource(PreviouslySelectedLevelUpScreenWidget(widget));
                }
                return;
            }
        }
        if !game_state_stack.contains(&GameState::LevelUpScreen) {
            commands.remove_resource::<LevelUpScreenReason>();
        }
    } else {
        commands.remove_resource::<PreviouslySelectedLevelUpScreenWidget>();
        commands.remove_resource::<LevelUpScreenReason>();
    }
    if let Ok(entity) = level_up_screen_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Rerolls perks offered in the level up screen.
pub fn reroll_perks(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    perk_registry: Res<PerkRegistry>,
    level_up_screen_configuration: Res<LevelUpScreenConfiguration>,
    level_up_screen_state: Option<ResMut<LevelUpScreenState>>,
) {
    let mut new_level_up_screen_state = match level_up_screen_state {
        Some(mut level_up_screen_state) => std::mem::take(level_up_screen_state.deref_mut()),
        None => LevelUpScreenState::default(),
    };

    let expected_number_of_perks = level_up_screen_configuration.number_of_perks as usize;
    let mut actual_number_of_perks = new_level_up_screen_state.offered_perk_ids.len();

    if expected_number_of_perks < actual_number_of_perks {
        // this can only happen if number of perks to offer
        // has changed when the level up screen is open.

        // in this case, number of perks to offer is reduced,
        // so we can just truncate the offered perk ids and call it a day.
        new_level_up_screen_state.offered_perk_ids.truncate(expected_number_of_perks);
    } else {
        if expected_number_of_perks == actual_number_of_perks {
            // we're doing a regular reroll, so we need to reset everything.
            new_level_up_screen_state.offered_perk_ids.clear();
            actual_number_of_perks = 0;
        }

        let mut commonness_of_perks_that_can_be_offered = Vec::new();
        for entry in perk_registry.iter() {
            let commonness = entry.perk.commonness;
            if commonness != 0 {
                commonness_of_perks_that_can_be_offered.push((entry.perk.id(), commonness));
            }
        }
        commonness_of_perks_that_can_be_offered.sort_by(|(id1, commonness1), (id2, commonness)| {
            if commonness1 == commonness {
                id1.cmp(id2)
            } else {
                commonness1.cmp(commonness).reverse()
            }
        });

        let additional_perks_to_offer = expected_number_of_perks - actual_number_of_perks;
        if commonness_of_perks_that_can_be_offered.is_empty() {
            log::error!(
                "unable to select {}{} perk{} to offer in the level up screen \
                as no perk is eligible to be offered in the level up screen",
                additional_perks_to_offer,
                if actual_number_of_perks == 0 { "" } else { " more" },
                if additional_perks_to_offer == 1 { "" } else { "s" },
            );
        } else if additional_perks_to_offer > 0 {
            let total_commonness = commonness_of_perks_that_can_be_offered
                .iter()
                .map(|(_, commonness)| commonness)
                .sum::<u64>();

            let mut probability_table = Table::new();
            probability_table.add_row(row![c -> "Perk", c -> "Chance", c -> "Probability"]);
            for (id, commonness) in commonness_of_perks_that_can_be_offered.iter() {
                probability_table.add_row(row![
                    l -> id,
                    r -> format!("({} / {})", commonness, total_commonness),
                    r -> format!(
                        "{:.6}%",
                        ((*commonness as f64) / (total_commonness as f64)) * 100.00,
                    )
                ]);
            }
            let probability_table = probability_table.to_string();

            log::info!(
                "{}{} perk{} to offer will be selected randomly with these probabilities:\n{}",
                additional_perks_to_offer,
                if actual_number_of_perks == 0 { "" } else { " more" },
                if additional_perks_to_offer == 1 { "" } else { "s" },
                probability_table.trim_end(),
            );

            for _ in 0..additional_perks_to_offer {
                match commonness_of_perks_that_can_be_offered
                    .choose_weighted(rng.deref_mut(), |(_, commonness)| *commonness)
                {
                    Ok((id, commonness)) => {
                        log::info!(
                            "offering randomly selected \"{}\" \
                            with {:.6}% probability ({} / {})",
                            id,
                            ((*commonness as f64) / (total_commonness as f64)) * 100.00,
                            commonness,
                            total_commonness,
                        );
                        new_level_up_screen_state.offered_perk_ids.push((*id).clone())
                    },
                    Err(error) => {
                        log::error!(
                            "unable to choose a random perk to offer in the level up screen ({})",
                            error,
                        );
                        break;
                    },
                }
            }
        }
    }

    commands.insert_resource(new_level_up_screen_state);
}


/// Updates offered perks container with offered perk containers.
pub fn update_offered_perks(
    mut commands: Commands,
    level_up_screen_perks_container_query: Query<Entity, With<LevelUpScreenPerksContainer>>,
    selected_widget_query: Query<Entity, With<WidgetSelected>>,
    asset_server: Res<AssetServer>,
    level_up_screen_state: Res<LevelUpScreenState>,
    mut level_up_screen_widgets: ResMut<LevelUpScreenWidgets>,
    perk_registry: Res<PerkRegistry>,
    localization: Res<Localization>,
) {
    let level_up_screen_perks_container_entity =
        match level_up_screen_perks_container_query.get_single() {
            Ok(query_result) => query_result,
            Err(_) => {
                return;
            },
        };

    let perk_name_style = styles::perk_name_text();
    let perk_name_colors = WidgetColors::container();
    let perk_name_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let perk_name_size = PERK_NAME_FONT_SIZE;

    let perk_description_style = styles::perk_description_text();
    let perk_description_colors = WidgetColors::container().text(Color::rgb(0.80, 0.80, 0.80));
    let perk_description_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let perk_description_size = PERK_DESCRIPTION_FONT_SIZE;

    let select_button_style = styles::select_button();
    let select_button_colors = WidgetColors::button();
    let select_button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let select_button_size = SELECT_BUTTON_FONT_SIZE;

    let mut children = Vec::new();
    let mut select_widgets = Vec::new();

    for (perk_index, perk_id) in level_up_screen_state.offered_perk_ids.iter().enumerate() {
        let perk_position = NonZeroUsize::new(perk_index + 1).unwrap();

        let perk_container = commands
            .spawn((
                Name::new(format!("Perk {}", perk_position)),
                LevelUpScreenPerkContainer,
                NodeBundle { style: styles::perk_container(), ..default() },
            ))
            .id();

        children.push(perk_container);

        let perk = match perk_registry.find_perk_by_id(perk_id) {
            Some(perk) => perk,
            None => {
                continue;
            },
        };

        let perk_details = {
            let perk_details = commands
                .spawn((
                    Name::new("Details"),
                    LevelUpScreenPerkDetails,
                    NodeBundle {
                        style: styles::perk_details(),
                        border_color: BorderColor(PERK_DETAILS_BORDER_COLOR),
                        background_color: BackgroundColor(PERK_DETAILS_BACKGROUND_COLOR),
                        ..default()
                    },
                ))
                .id();

            let item_name = commands
                .spawn((
                    Name::new("Name"),
                    LevelUpScreenPerkNameText,
                    TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                perk.name.get(&localization),
                                TextStyle {
                                    font: perk_name_font.clone(),
                                    font_size: perk_name_size,
                                    color: perk_name_colors.text,
                                },
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        style: perk_name_style.clone(),
                        ..default()
                    },
                    perk.name.clone(),
                ))
                .id();

            let item_description = commands
                .spawn((
                    Name::new("Description"),
                    LevelUpScreenPerkDescriptionText,
                    TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                perk.description.get(&localization),
                                TextStyle {
                                    font: perk_description_font.clone(),
                                    font_size: perk_description_size,
                                    color: perk_description_colors.text,
                                },
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        style: perk_description_style.clone(),
                        ..default()
                    },
                    perk.description.clone(),
                ))
                .id();

            commands.entity(perk_details).add_child(item_name).add_child(item_description);

            perk_details
        };

        let select_button = Widget::button(
            &mut commands,
            (
                Name::new("Select Button"),
                LevelUpScreenSelectButton { perk: perk.deref().clone() },
                Widget::default(),
            ),
            &select_button_style,
            select_button_colors,
            &select_button_font,
            select_button_size,
            localization::select_button(),
            &localization,
        );

        select_widgets.push(select_button);

        commands.entity(perk_container).add_child(perk_details).add_child(select_button);
    }

    if let Ok(selected_widget) = selected_widget_query.get_single() {
        let mut selected_widget_index = None;
        for (i, widgets) in level_up_screen_widgets.iter().enumerate() {
            for (j, widget) in widgets.iter().cloned().enumerate() {
                if widget == selected_widget {
                    selected_widget_index = Some((i, j));
                    break;
                }
            }
        }
        if let Some((i, j)) = selected_widget_index {
            if i == 0 {
                commands
                    .entity(
                        select_widgets
                            .get(j)
                            .cloned()
                            .or(select_widgets.last().cloned())
                            .unwrap_or(level_up_screen_widgets[1].last().cloned().unwrap()),
                    )
                    .insert(WidgetSelected::now());
            }
        }
    }

    let mut level_up_screen_perks_container =
        commands.entity(level_up_screen_perks_container_entity);
    level_up_screen_perks_container.despawn_descendants();

    for child in children {
        level_up_screen_perks_container.add_child(child);
    }

    level_up_screen_widgets[0] = select_widgets;
}

/// Updates level up screen widgets with appropriate widget up/down/left/right components.
pub fn update_level_up_screen_widget_hierarchy(
    mut commands: Commands,
    widget_query: Query<&GlobalTransform, With<Widget>>,
    level_up_screen_widgets: Res<LevelUpScreenWidgets>,
) {
    for current_widgets_index in 0..level_up_screen_widgets.len() {
        let mut up_widgets_index = if current_widgets_index != 0 {
            current_widgets_index - 1
        } else {
            current_widgets_index
        };
        if up_widgets_index != 0 {
            while level_up_screen_widgets[up_widgets_index].is_empty() {
                up_widgets_index -= 1;
                if up_widgets_index == 0 {
                    break;
                }
            }
        }
        if level_up_screen_widgets[up_widgets_index].is_empty() {
            up_widgets_index = current_widgets_index;
        }

        let mut down_widgets_index = if current_widgets_index != level_up_screen_widgets.len() - 1 {
            current_widgets_index + 1
        } else {
            current_widgets_index
        };
        if down_widgets_index != level_up_screen_widgets.len() - 1 {
            while level_up_screen_widgets[down_widgets_index].is_empty() {
                down_widgets_index += 1;
                if down_widgets_index == level_up_screen_widgets.len() - 1 {
                    break;
                }
            }
        }
        if level_up_screen_widgets[down_widgets_index].is_empty() {
            down_widgets_index = current_widgets_index;
        }

        let current_widgets = &level_up_screen_widgets[current_widgets_index];
        for current_widget_index in 0..current_widgets.len() {
            let left_widget = if current_widget_index != 0 {
                current_widgets[current_widget_index - 1]
            } else {
                current_widgets[current_widget_index]
            };
            let current_widget = current_widgets[current_widget_index];
            let right_widget = if current_widget_index != current_widgets.len() - 1 {
                current_widgets[current_widget_index + 1]
            } else {
                current_widgets[current_widget_index]
            };

            let up_widget = if up_widgets_index == current_widgets_index {
                current_widget
            } else {
                let up_widgets = &level_up_screen_widgets[up_widgets_index];
                assert!(!up_widgets.is_empty());

                match widget_query.get(current_widget) {
                    Ok(current_widget_transform) => {
                        let mut up_widget = up_widgets[0];
                        let mut best_distance = f32::INFINITY;

                        for up_widget_candidate in up_widgets.iter().cloned() {
                            match widget_query.get(up_widget_candidate) {
                                Ok(up_widget_candidate_transform) => {
                                    let distance = (up_widget_candidate_transform.translation()
                                        - current_widget_transform.translation())
                                    .length();
                                    if distance < best_distance {
                                        up_widget = up_widget_candidate;
                                        best_distance = distance;
                                    }
                                },
                                Err(_) => continue,
                            }
                        }

                        up_widget
                    },
                    Err(_) => up_widgets[0],
                }
            };
            let down_widget = if down_widgets_index == current_widgets_index {
                current_widget
            } else {
                let down_widgets = &level_up_screen_widgets[down_widgets_index];
                assert!(!down_widgets.is_empty());

                match widget_query.get(current_widget) {
                    Ok(current_widget_transform) => {
                        let mut down_widget = down_widgets[0];
                        let mut best_distance = f32::INFINITY;

                        for down_widget_candidate in down_widgets.iter().cloned() {
                            match widget_query.get(down_widget_candidate) {
                                Ok(down_widget_candidate_transform) => {
                                    let distance = (down_widget_candidate_transform.translation()
                                        - current_widget_transform.translation())
                                    .length();
                                    if distance < best_distance {
                                        down_widget = down_widget_candidate;
                                        best_distance = distance;
                                    }
                                },
                                Err(_) => continue,
                            }
                        }

                        down_widget
                    },
                    Err(_) => down_widgets[0],
                }
            };

            commands.entity(current_widget).insert((
                WidgetUp(up_widget),
                WidgetDown(down_widget),
                WidgetLeft(left_widget),
                WidgetRight(right_widget),
            ));
        }
    }
}


/// Updates balance text.
pub fn update_balance_text(
    mut balance_text_query: Query<&mut Text, With<LevelUpScreenBalanceText>>,
    balance: Res<Balance>,
) {
    let mut balance_text = match balance_text_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    balance_text.sections[0].value = format!("{}", *balance);
}

/// Updates reroll button.
pub fn update_reroll_button(
    mut commands: Commands,
    mut reroll_button_query: Query<(Entity, &mut LevelUpScreenRerollButton)>,
    mut text_query: Query<(&Parent, &mut LocalizedText)>,
    balance: Res<Balance>,
    level_up_screen_configuration: Res<LevelUpScreenConfiguration>,
) {
    let (reroll_button_entity, mut reroll_button) = match reroll_button_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let reroll_cost = level_up_screen_configuration.reroll_cost();
    reroll_button.cost = reroll_cost;

    for (parent_entity, mut refresh_button_text) in text_query.iter_mut() {
        if parent_entity.get() == reroll_button_entity {
            *refresh_button_text = localization::reroll_button(reroll_cost);
            break;
        }
    }

    if *balance < reroll_cost {
        commands.entity(reroll_button_entity).insert(WidgetDisabled);
    } else {
        commands.entity(reroll_button_entity).remove::<WidgetDisabled>();
    }
}


/// Navigates the level up screen using level up screen actions.
pub fn navigation(
    mut commands: Commands,
    mut level_up_screen_query: Query<&ActionState<LevelUpScreenAction>, With<LevelUpScreen>>,
    mut selected_widget_query: Query<
        (&mut Widget, &WidgetUp, &WidgetDown, &WidgetLeft, &WidgetRight),
        With<WidgetSelected>,
    >,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let level_up_screen_action_state = match level_up_screen_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if level_up_screen_action_state.just_pressed(&LevelUpScreenAction::Pause) {
        game_state_stack.push(GameState::Paused);
        next_game_state.set(GameState::Transition);
        return;
    }

    let (mut selected_widget, up_widget, down_widget, left_widget, right_widget) =
        match selected_widget_query.get_single_mut() {
            Ok(query_result) => query_result,
            Err(_) => return,
        };

    if level_up_screen_action_state.just_pressed(&LevelUpScreenAction::Select) {
        selected_widget.clicked = true;
        return;
    }

    let go_up = level_up_screen_action_state.just_pressed(&LevelUpScreenAction::Up);
    let go_down = level_up_screen_action_state.just_pressed(&LevelUpScreenAction::Down);

    if (go_up || go_down) && !(go_up && go_down) {
        if go_down {
            commands.entity(down_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(up_widget.0).insert(WidgetSelected::now());
        }
    }

    let go_left = level_up_screen_action_state.just_pressed(&LevelUpScreenAction::Left);
    let go_right = level_up_screen_action_state.just_pressed(&LevelUpScreenAction::Right);

    if (go_left || go_right) && !(go_left && go_right) {
        if go_right {
            commands.entity(right_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(left_widget.0).insert(WidgetSelected::now());
        }
    }
}


/// Obtains the selected perk and transitions to the next state.
pub fn select_button_interaction(
    mut commands: Commands,
    mut select_button_query: Query<(&mut Widget, &LevelUpScreenSelectButton), Changed<Widget>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut level_up_screen_reason: ResMut<LevelUpScreenReason>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok((mut button, metadata)) = select_button_query.get_single_mut() {
        button.on_click(|| {
            let obtain_lose_perk_reason = match level_up_screen_reason.deref_mut() {
                LevelUpScreenReason::LevelingUp { to } => {
                    let result = ObtainLosePerkReason::LevelingUp { to: *to };
                    to.0 = to.0.checked_add(1).unwrap_or(NonZeroU16::MAX);
                    result
                },
                LevelUpScreenReason::Cheating => ObtainLosePerkReason::Cheating,
            };

            commands.remove_resource::<LevelUpScreenState>();
            commands.run_system_with_input(
                registered_systems.perk.obtain_perk,
                (metadata.perk.clone(), obtain_lose_perk_reason),
            );

            game_state_stack.pop();
            next_game_state.set(GameState::Transition);
        });
    }
}

/// Rerolls offered perks in the level up screen.
pub fn reroll_button_interaction(
    mut commands: Commands,
    mut reroll_button_query: Query<(&mut Widget, &LevelUpScreenRerollButton), Changed<Widget>>,
    mut level_up_screen_configuration: ResMut<LevelUpScreenConfiguration>,
    mut balance: ResMut<Balance>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok((mut button, metadata)) = reroll_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("reroll button is clicked");

            let reroll_cost = metadata.cost;
            if *balance < reroll_cost {
                log::error!(
                    "unable to reroll the perks offered in the level up screen, \
                    which required {} experience, but only {} experience was available",
                    reroll_cost,
                    *balance,
                );
                return;
            }

            if *reroll_cost != 0.00 {
                balance.spend(reroll_cost, "reroll the perks in the level up screen");
            }
            commands.run_system(registered_systems.level_up_screen.reroll_perks);

            level_up_screen_configuration.reroll_cost.step();
            log::info!("new reroll cost is {}", level_up_screen_configuration.reroll_cost());
        });
    }
}


/// Resets the level up screen.
pub fn reset_level_up_screen_configuration(mut commands: Commands) {
    commands.insert_resource(LevelUpScreenConfiguration::default());
}
