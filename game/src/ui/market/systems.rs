use crate::{
    prelude::*,
    ui::market::{
        constants::*,
        styles,
    },
};


/// Spawns the market.
pub fn spawn_market(
    mut commands: Commands,
    market_query: Query<&mut Visibility, With<Market>>,
    asset_server: Res<AssetServer>,
    balance: Res<Balance>,
    market_action_input_map: Res<InputMap<MarketAction>>,
    market_configuration: Res<MarketConfiguration>,
    market_state: Res<MarketState>,
    previously_selected_widget: Option<Res<PreviouslySelectedWidget>>,
) {
    if !market_query.is_empty() {
        if let Some(previously_selected_widget) = previously_selected_widget {
            commands.entity(previously_selected_widget.0).insert(WidgetSelected::now());
            commands.remove_resource::<PreviouslySelectedWidget>();
        }
        return;
    }

    // Create market action state.
    let mut action_state = ActionState::default();
    {
        let pressed = ActionData { state: ButtonState::Pressed, ..default() };

        action_state.set_action_data(MarketAction::Pause, pressed.clone());
        action_state.set_action_data(MarketAction::Close, pressed.clone());
        action_state.set_action_data(MarketAction::Up, pressed.clone());
        action_state.set_action_data(MarketAction::Down, pressed.clone());
        action_state.set_action_data(MarketAction::Left, pressed.clone());
        action_state.set_action_data(MarketAction::Right, pressed.clone());
        action_state.set_action_data(MarketAction::Select, pressed);
    }

    // Childrens and widgets.
    let mut market_children = Vec::new();
    let mut market_widgets = MarketWidgets::default();

    // Root.
    let market = {
        commands
            .spawn((
                Name::new("Market"),
                Market,
                InputManagerBundle::<MarketAction> {
                    action_state,
                    input_map: market_action_input_map.clone(),
                },
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Percent(ROW_GAP_PERCENT),
                        ..default()
                    },
                    background_color: BackgroundColor(BACKGROUND_COLOR),
                    ..default()
                },
            ))
            .id()
    };

    // Header container.
    {
        let header_container_style = styles::header_container();
        let header_container_colors = WidgetColors::container();

        let header_container = commands
            .spawn((
                Name::new("Header"),
                MarketHeaderContainer,
                NodeBundle {
                    style: header_container_style,
                    background_color: header_container_colors.normal.into(),
                    ..default()
                },
                header_container_colors,
            ))
            .id();

        market_children.push(header_container);

        {
            // Balance Container.
            let balance_container = {
                let balance_container_style = styles::balance_container();
                let balance_container_colors = WidgetColors::container();

                let balance_container = Widget::container(
                    &mut commands,
                    (Name::new("Balance"), MarketBalanceContainer, Widget::default()),
                    &balance_container_style,
                    balance_container_colors,
                );

                let text_style = styles::balance_text();
                let text_font = asset_server.load("fonts/FiraSans-Bold.ttf");
                let text_size = BALANCE_TEXT_FONT_SIZE;

                let balance_text = commands
                    .spawn((
                        Name::new("Text"),
                        MarketBalanceText,
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

            // Refresh button.
            let refresh_button = {
                let refresh_cost = market_configuration.refresh_cost(&market_state);

                let refresh_button_style = styles::refresh_button();
                let refresh_button_colors = WidgetColors::button();
                let refresh_button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
                let refresh_button_size = REFRESH_BUTTON_FONT_SIZE;

                let refresh_button = Widget::button(
                    &mut commands,
                    (
                        Name::new("Refresh Button"),
                        MarketRefreshButton { cost: refresh_cost },
                        Widget::default(),
                    ),
                    &refresh_button_style,
                    refresh_button_colors,
                    &refresh_button_font,
                    refresh_button_size,
                    format!("Refresh - {}", market_configuration.refresh_cost(&market_state)),
                );

                let raw_refresh_cost = if market_configuration.free_refreshes > 0 {
                    Balance::ZERO
                } else {
                    market_configuration.refresh_cost.get()
                };
                let market_is_initialized = market_state.offered_item_ids.len()
                    == (market_configuration.number_of_items as usize);

                if (!market_is_initialized && *balance < raw_refresh_cost)
                    || (market_is_initialized && *balance < refresh_cost)
                {
                    commands.entity(refresh_button).insert(WidgetDisabled);
                }

                refresh_button
            };

            commands
                .entity(header_container)
                .add_child(balance_container)
                .add_child(refresh_button);
            market_widgets[0] = vec![balance_container, refresh_button];
        }
    }

    // Items.
    {
        let items_container_style = styles::items_container();

        let items_container = commands
            .spawn((
                Name::new("Items"),
                MarketItemsContainer,
                NodeBundle { style: items_container_style, ..default() },
            ))
            .id();

        market_children.push(items_container);
    }

    // Continue button.
    {
        let continue_button_style = styles::continue_button();
        let continue_button_colors = WidgetColors::button();
        let continue_button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let continue_button_size = CONTINUE_BUTTON_FONT_SIZE;

        let continue_button = Widget::button(
            &mut commands,
            (
                Name::new("Continue Button"),
                MarketContinueButton,
                Widget::default().selected(),
                WidgetSelected::now(),
            ),
            &continue_button_style,
            continue_button_colors,
            &continue_button_font,
            continue_button_size,
            "Continue",
        );

        market_children.push(continue_button);
        market_widgets[3] = vec![continue_button];
    }

    // Add children.
    let mut market_entity = commands.entity(market);
    for child in market_children {
        market_entity.add_child(child);
    }

    // Insert widgets.
    commands.insert_resource(market_widgets);
}

/// Despawns the market.
pub fn despawn_market(
    mut commands: Commands,
    market_query: Query<Entity, With<Market>>,
    widget_query: Query<Entity, With<WidgetSelected>>,
    game_state_stack: Res<GameStateStack>,
    app_state: Res<State<AppState>>,
) {
    if app_state.get() == &AppState::Game {
        if game_state_stack.contains(&GameState::Market) {
            if let Ok(widget) = widget_query.get_single() {
                commands.insert_resource(PreviouslySelectedWidget(widget));
            }
            return;
        }
    } else {
        commands.remove_resource::<PreviouslySelectedWidget>();
    }
    if let Ok(entity) = market_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<MarketWidgets>();
}


/// Updates balance text.
pub fn update_balance_text(
    mut balance_text_query: Query<&mut Text, With<MarketBalanceText>>,
    balance: Res<Balance>,
) {
    let mut balance_text = match balance_text_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    balance_text.sections[0].value = format!("{}", *balance);
}

/// Updates refresh button.
pub fn update_refresh_button(
    mut commands: Commands,
    mut refresh_button_query: Query<(Entity, &mut MarketRefreshButton)>,
    mut text_query: Query<(&Parent, &mut Text)>,
    balance: Res<Balance>,
    market_configuration: Res<MarketConfiguration>,
    market_state: Res<MarketState>,
) {
    let (refresh_button_entity, mut refresh_button) = match refresh_button_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let refresh_cost = market_configuration.refresh_cost(&market_state);
    refresh_button.cost = refresh_cost;

    for (parent_entity, mut refresh_button_text) in text_query.iter_mut() {
        if parent_entity.get() == refresh_button_entity {
            refresh_button_text.sections[0].value = format!("Refresh - {}", refresh_cost);
            break;
        }
    }

    if *balance < refresh_cost {
        commands.entity(refresh_button_entity).insert(WidgetDisabled);
    } else {
        commands.entity(refresh_button_entity).remove::<WidgetDisabled>();
    }
}


/// Updates offered items container with offered item containers.
pub fn update_offered_items(
    mut commands: Commands,
    market_items_container_query: Query<Entity, With<MarketItemsContainer>>,
    selected_widget_query: Query<Entity, With<WidgetSelected>>,
    asset_server: Res<AssetServer>,
    balance: Res<Balance>,
    market_state: Res<MarketState>,
    mut market_widgets: ResMut<MarketWidgets>,
    item_registry: Res<ItemRegistry>,
) {
    let market_items_container_entity = match market_items_container_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => {
            return;
        },
    };

    let item_container_style = styles::item_container();
    let item_details_style = styles::item_details();

    let item_name_style = styles::item_name_text();
    let item_name_colors = WidgetColors::container();
    let item_name_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let item_name_size = ITEM_NAME_FONT_SIZE;

    let buy_button_style = styles::buy_button();
    let buy_button_colors = WidgetColors::button();
    let buy_button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let buy_button_size = BUY_BUTTON_FONT_SIZE;

    let lock_button_style = styles::lock_button();
    let lock_button_colors = WidgetColors::button();
    let lock_button_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let lock_button_size = LOCK_BUTTON_FONT_SIZE;

    let mut children = Vec::new();
    let mut buy_widgets = Vec::new();
    let mut lock_widgets = Vec::new();

    for (item_index, item_id) in market_state.offered_item_ids.iter().enumerate() {
        let item_position = NonZeroUsize::new(item_index + 1).unwrap();

        let item_container = commands
            .spawn((
                Name::new(format!("Item {}", item_position)),
                MarketItemContainer,
                NodeBundle { style: item_container_style.clone(), ..default() },
            ))
            .id();

        children.push(item_container);

        if market_state.is_acquired(item_position) {
            continue;
        }

        let item = match item_registry.find_item_by_id(item_id) {
            Some(item) => item,
            None => {
                continue;
            },
        };

        let item_details = {
            let item_details = commands
                .spawn((
                    Name::new("Details"),
                    MarketItemDetails,
                    NodeBundle {
                        style: item_details_style.clone(),
                        border_color: BorderColor(ITEM_DETAILS_BORDER_COLOR),
                        background_color: BackgroundColor(ITEM_DETAILS_BACKGROUND_COLOR),
                        ..default()
                    },
                ))
                .id();

            let item_name = commands
                .spawn((
                    Name::new("Name"),
                    MarketItemNameText,
                    TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                item.name(),
                                TextStyle {
                                    font: item_name_font.clone(),
                                    font_size: item_name_size,
                                    color: item_name_colors.text,
                                },
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        style: item_name_style.clone(),
                        ..default()
                    },
                ))
                .id();

            let price = item.base_price;
            let buy_button_label = format!("{}", price);

            let buy_button = Widget::button(
                &mut commands,
                (Name::new("Buy Button"), MarketBuyButton { item_index, price }, Widget::default()),
                &buy_button_style,
                buy_button_colors,
                &buy_button_font,
                buy_button_size,
                buy_button_label,
            );

            buy_widgets.push(buy_button);

            if *balance < price {
                commands.entity(buy_button).insert(WidgetDisabled);
            } else {
                commands.entity(buy_button).remove::<WidgetDisabled>();
            }

            commands.entity(item_details).add_child(item_name).add_child(buy_button);

            item_details
        };

        let lock_button = Widget::button(
            &mut commands,
            (Name::new("Lock Button"), MarketLockButton { item_index }, Widget::default()),
            &lock_button_style,
            lock_button_colors,
            &lock_button_font,
            lock_button_size,
            if market_state.is_locked(item_position) { "Unlock" } else { "Lock" },
        );

        lock_widgets.push(lock_button);

        commands.entity(item_container).add_child(item_details).add_child(lock_button);
    }

    if let Ok(selected_widget) = selected_widget_query.get_single() {
        let mut selected_widget_index = None;
        for (i, widgets) in market_widgets.iter().enumerate() {
            for (j, widget) in widgets.iter().cloned().enumerate() {
                if widget == selected_widget {
                    selected_widget_index = Some((i, j));
                    break;
                }
            }
        }
        if let Some((i, j)) = selected_widget_index {
            if i == 1 {
                commands
                    .entity(
                        buy_widgets
                            .get(j)
                            .cloned()
                            .or(buy_widgets.last().cloned())
                            .unwrap_or(market_widgets[3].last().cloned().unwrap()),
                    )
                    .insert(WidgetSelected::now());
            }
            if i == 2 {
                commands
                    .entity(
                        lock_widgets
                            .get(j)
                            .cloned()
                            .or(lock_widgets.last().cloned())
                            .unwrap_or(market_widgets[3].last().cloned().unwrap()),
                    )
                    .insert(WidgetSelected::now());
            }
        }
    }

    let mut market_items_container = commands.entity(market_items_container_entity);
    market_items_container.despawn_descendants();

    for child in children {
        market_items_container.add_child(child);
    }

    market_widgets[1] = buy_widgets;
    market_widgets[2] = lock_widgets;
}

/// Updates market widgets with appropriate widget up/down/left/right components.
pub fn update_market_widget_hierarchy(
    mut commands: Commands,
    widget_query: Query<&GlobalTransform, With<Widget>>,
    market_widgets: Res<MarketWidgets>,
) {
    for current_widgets_index in 0..market_widgets.len() {
        let mut up_widgets_index = if current_widgets_index != 0 {
            current_widgets_index - 1
        } else {
            current_widgets_index
        };
        if up_widgets_index != 0 {
            while market_widgets[up_widgets_index].is_empty() {
                up_widgets_index -= 1;
                if up_widgets_index == 0 {
                    break;
                }
            }
        }
        if market_widgets[up_widgets_index].is_empty() {
            up_widgets_index = current_widgets_index;
        }

        let mut down_widgets_index = if current_widgets_index != market_widgets.len() - 1 {
            current_widgets_index + 1
        } else {
            current_widgets_index
        };
        if down_widgets_index != market_widgets.len() - 1 {
            while market_widgets[down_widgets_index].is_empty() {
                down_widgets_index += 1;
                if down_widgets_index == market_widgets.len() - 1 {
                    break;
                }
            }
        }
        if market_widgets[down_widgets_index].is_empty() {
            down_widgets_index = current_widgets_index;
        }

        let current_widgets = &market_widgets[current_widgets_index];
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
                let up_widgets = &market_widgets[up_widgets_index];
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
                let down_widgets = &market_widgets[down_widgets_index];
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


/// Navigates the market using market actions.
pub fn navigation(
    mut commands: Commands,
    mut market_query: Query<&ActionState<MarketAction>, With<Market>>,
    mut selected_widget_query: Query<
        (&mut Widget, &WidgetUp, &WidgetDown, &WidgetLeft, &WidgetRight),
        With<WidgetSelected>,
    >,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let market_action_state = match market_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if market_action_state.just_pressed(&MarketAction::Pause) {
        game_state_stack.push(GameState::Paused);
        next_game_state.set(GameState::Transition);
        return;
    }

    if market_action_state.just_pressed(&MarketAction::Close) {
        log::info!("closing the market");
        game_state_stack.pop();
        next_game_state.set(GameState::Transition);
        return;
    }

    let (mut selected_widget, up_widget, down_widget, left_widget, right_widget) =
        match selected_widget_query.get_single_mut() {
            Ok(query_result) => query_result,
            Err(_) => return,
        };

    if market_action_state.just_pressed(&MarketAction::Select) {
        selected_widget.clicked = true;
        return;
    }

    let go_up = market_action_state.just_pressed(&MarketAction::Up);
    let go_down = market_action_state.just_pressed(&MarketAction::Down);

    if (go_up || go_down) && !(go_up && go_down) {
        if go_down {
            commands.entity(down_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(up_widget.0).insert(WidgetSelected::now());
        }
    }

    let go_left = market_action_state.just_pressed(&MarketAction::Left);
    let go_right = market_action_state.just_pressed(&MarketAction::Right);

    if (go_left || go_right) && !(go_left && go_right) {
        if go_right {
            commands.entity(right_widget.0).insert(WidgetSelected::now());
        } else {
            commands.entity(left_widget.0).insert(WidgetSelected::now());
        }
    }
}


/// Buys an item.
pub fn buy_button_interaction(
    mut buy_button_query: Query<(&mut Widget, &MarketBuyButton), Changed<Widget>>,
    mut balance: ResMut<Balance>,
    mut market_state: ResMut<MarketState>,
) {
    for (mut button, metadata) in buy_button_query.iter_mut() {
        button.on_click(|| {
            let item_position = NonZeroUsize::new(metadata.item_index + 1).unwrap();
            let item_cost = metadata.price;

            if *balance < item_cost {
                log::error!(
                    "unable to buy item {} in the market, which required {} experience, \
                    but only {} experience was available",
                    item_position,
                    item_cost,
                    *balance,
                );
                return;
            }

            balance.spend(item_cost, format!("buy item {} in the market", item_position));
            market_state.acquire(item_position);
        });
    }
}

/// (Un)locks an item.
pub fn lock_button_interaction(
    mut lock_button_query: Query<(&mut Widget, &MarketLockButton), Changed<Widget>>,
    mut market_state: ResMut<MarketState>,
) {
    for (mut button, metadata) in lock_button_query.iter_mut() {
        button.on_click(|| {
            let item_position = NonZeroUsize::new(metadata.item_index + 1).unwrap();
            if market_state.is_locked(item_position) {
                market_state.unlock(item_position);
            } else {
                market_state.lock(item_position);
            }
        });
    }
}

/// Refreshes the market.
pub fn refresh_button_interaction(
    mut commands: Commands,
    mut refresh_button_query: Query<(&mut Widget, &MarketRefreshButton), Changed<Widget>>,
    mut market_configuration: ResMut<MarketConfiguration>,
    mut balance: ResMut<Balance>,
    market_state: ResMut<MarketState>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok((mut button, metadata)) = refresh_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("refresh button is clicked");

            let refresh_cost = metadata.cost;
            if *balance < refresh_cost {
                log::error!(
                    "unable to refresh the market, which required {} experience, \
                    but only {} experience was available",
                    refresh_cost,
                    *balance,
                );
                return;
            }

            let refresh_was_free_as_no_item_was_available =
                market_configuration.refresh_is_free_as_no_item_is_available(&market_state);
            let free_refresh_used = (market_configuration.free_refreshes > 0)
                && !refresh_was_free_as_no_item_was_available;

            if *refresh_cost != 0.00 {
                balance.spend(refresh_cost, "refresh the market");
            } else if free_refresh_used {
                if market_configuration.free_refreshes == 1 {
                    log::info!("using the last free refresh");
                } else {
                    log::info!(
                        "using 1 of {} available free refreshes",
                        market_configuration.free_refreshes,
                    );
                }
            } else {
                log::info!("refreshing for free as no item is available to purchase in the market");
            }
            commands.run_system(registered_systems.market.refresh_market);

            if free_refresh_used {
                market_configuration.free_refreshes -= 1;
            }

            if !(refresh_was_free_as_no_item_was_available || free_refresh_used) {
                market_configuration.refresh_cost.step();
                log::info!("new refresh cost is {}", market_configuration.refresh_cost.get());
            }
        });
    }
}

/// Closes the market.
pub fn continue_button_interaction(
    mut continue_button_query: Query<&mut Widget, (Changed<Widget>, With<MarketContinueButton>)>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = continue_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("closing the market");
            game_state_stack.pop();
            next_game_state.set(GameState::Transition);
        });
    }
}
