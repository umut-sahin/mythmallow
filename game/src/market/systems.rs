use crate::{
    market::commands::*,
    prelude::*,
};


/// Applies the market console commands.
pub fn apply_market_command(
    mut commands: Commands,
    mut balance: ResMut<Balance>,
    mut market_configuration: ResMut<MarketConfiguration>,
    mut market_state: ResMut<MarketState>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
    registered_systems: Res<RegisteredSystems>,
    item_registry: Res<ItemRegistry>,
    mut command: ConsoleCommand<MarketCommand>,
) {
    if let Some(Ok(MarketCommand { subcommand })) = command.take() {
        if *app_state.get() != AppState::Game {
            reply!(command, "Not available outside the game.");
            reply!(command, "");
            return;
        }

        match subcommand {
            MarketCommands::Show => {
                if market_state.offered_item_ids.is_empty() {
                    reply!(command, "Market is empty.");
                } else {
                    for (index, id) in market_state.offered_item_ids.iter().enumerate() {
                        let position = NonZeroUsize::new(index + 1).unwrap();
                        reply!(
                            command,
                            "{}) {} {}",
                            position,
                            id,
                            if market_state.is_acquired(position) {
                                "(acquired)"
                            } else if market_state.is_locked(position) {
                                "(locked)"
                            } else {
                                ""
                            },
                        );
                    }
                }
            },
            MarketCommands::Refresh => {
                commands.run_system(registered_systems.market.refresh_market);
                reply!(command, "Refreshed.");
            },
            MarketCommands::Lock { position } => {
                match market_state.lock(position) {
                    LockUnlockStatus::NotExist => {
                        reply!(command, "Failed to lock item {} as it doesn't exist.", position);
                    },
                    LockUnlockStatus::Acquired => {
                        reply!(command, "Failed to lock item {} as it's acquired.", position);
                    },
                    LockUnlockStatus::AlreadyLocked => {
                        reply!(command, "Failed to lock item {} as it's already locked.", position);
                    },
                    LockUnlockStatus::Locked => {
                        reply!(command, "Locked.");
                    },
                    _ => unreachable!(),
                }
            },
            MarketCommands::Unlock { position } => {
                match market_state.unlock(position) {
                    LockUnlockStatus::NotExist => {
                        reply!(command, "Failed to unlock item {} as it doesn't exist.", position);
                    },
                    LockUnlockStatus::Acquired => {
                        reply!(command, "Failed to unlock item {} as it's acquired.", position);
                    },
                    LockUnlockStatus::AlreadyUnlocked => {
                        reply!(command, "Failed to unlock item {} as it's not locked.", position);
                    },
                    LockUnlockStatus::Unlocked => {
                        reply!(command, "Unlocked.");
                    },
                    _ => unreachable!(),
                }
            },
            MarketCommands::Acquire { position } => {
                match market_state.acquire(position) {
                    AcquireStatus::NotExist => {
                        reply!(command, "Failed to acquire item {} as it doesn't exist.", position);
                    },
                    AcquireStatus::AlreadyAcquired => {
                        reply!(
                            command,
                            "Failed to acquire item {} as it's already acquired.",
                            position,
                        );
                    },
                    AcquireStatus::Acquired => {
                        reply!(command, "Acquired.");
                    },
                }
            },
            MarketCommands::Offer { position, item } => {
                let index = position.get() - 1;
                if market_state.offered_item_ids.len() <= index {
                    reply!(command, "Failed to offer item {} as it doesn't exist.", position);
                    reply!(command, "");
                    return;
                }

                if item_registry.find_item_by_id(&item).is_none() {
                    reply!(command, "Failed to offer {:?} as it doesn't exist.", item);
                    reply!(command, "");
                    return;
                }

                log::info!("offering {:?} as item {} in the market", item, position);
                market_state.offered_item_ids[index] = item;
                reply!(command, "Done.");
            },
            MarketCommands::Open => {
                match game_state.get() {
                    GameState::Playing => {
                        log::info!("opening the market");
                        game_state_stack.push(GameState::Market);
                        next_game_state.set(GameState::Transition);
                        reply!(command, "Opened.");
                    },
                    GameState::Market => {
                        reply!(command, "Already opened.");
                    },
                    GameState::LevelUpScreen => {
                        reply!(command, "Not available in the level up screen.");
                    },
                    GameState::Paused => {
                        reply!(command, "Not available in the pause menu.");
                    },
                    _ => {
                        reply!(command, "How did you time this, seriously?");
                    },
                }
            },
            MarketCommands::Close => {
                match game_state.get() {
                    GameState::Market => {
                        log::info!("closing the market");
                        game_state_stack.pop();
                        next_game_state.set(GameState::Transition);
                        reply!(command, "Closed.");
                    },
                    GameState::Playing => {
                        reply!(command, "Already closed.");
                    },
                    GameState::LevelUpScreen => {
                        reply!(command, "Not available in the level up screen.");
                    },
                    GameState::Paused => {
                        reply!(command, "Not available in the pause menu.");
                    },
                    _ => {
                        reply!(command, "How did you time this, seriously?");
                    },
                }
            },
            MarketCommands::Balance { subcommand } => {
                match subcommand {
                    BalanceCommands::Show => {
                        reply!(command, "{}", *balance);
                    },
                    BalanceCommands::Set { amount } => {
                        balance.set(Balance(amount));
                        reply!(command, "Done.");
                    },
                    BalanceCommands::Add { amount } => {
                        balance.gain(Balance(amount), "cheating :)");
                        reply!(command, "Done.");
                    },
                    BalanceCommands::Remove { amount } => {
                        balance.spend(Balance(amount), "cheat :|");
                        reply!(command, "Done.");
                    },
                }
            },
            MarketCommands::NumberOfItems { subcommand } => {
                match subcommand {
                    NumberOfItemsCommands::Show => {
                        reply!(command, "{}", market_configuration.number_of_items);
                    },
                    NumberOfItemsCommands::Set { number_of_items } => {
                        log::info!(
                            "setting the number of items in the market to {}",
                            number_of_items,
                        );
                        market_configuration.number_of_items = number_of_items;
                        reply!(command, "Done.");
                    },
                }
            },
            MarketCommands::RefreshCost { subcommand } => {
                match subcommand {
                    RefreshCostCommands::Show => {
                        reply!(command, "{}", market_configuration.refresh_cost(&market_state));
                    },
                    RefreshCostCommands::Model { subcommand } => {
                        match subcommand {
                            RefreshCostModelCommands::Show => {
                                reply!(command, "{}", market_configuration.refresh_cost);
                            },
                            RefreshCostModelCommands::Set { subcommand } => {
                                match subcommand {
                                    RefreshCostModelSetCommands::Constant { cost } => {
                                        market_configuration.refresh_cost =
                                            MarketRefreshCost::constant(Balance(cost));
                                    },
                                    RefreshCostModelSetCommands::Linear {
                                        base,
                                        increase,
                                        step,
                                        max,
                                    } => {
                                        market_configuration.refresh_cost =
                                            MarketRefreshCost::linear(
                                                Balance(base),
                                                Balance(increase),
                                                max.map(Balance),
                                            );
                                        if let Some(step) = step {
                                            market_configuration.refresh_cost.set_step(step);
                                        }
                                    },
                                    RefreshCostModelSetCommands::Exponential {
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
                                        market_configuration.refresh_cost =
                                            MarketRefreshCost::exponential(
                                                Balance(base),
                                                factor,
                                                max.map(Balance),
                                            );
                                        if let Some(step) = step {
                                            market_configuration.refresh_cost.set_step(step);
                                        }
                                    },
                                }
                                log::info!(
                                    "setting the refresh cost model of the market to {}",
                                    market_configuration.refresh_cost,
                                );
                                reply!(command, "Done.");
                            },
                        }
                    },
                }
            },
            MarketCommands::FreeRefreshes { subcommand } => {
                match subcommand {
                    FreeRefreshesCommands::Show => {
                        reply!(command, "{}", market_configuration.free_refreshes);
                    },
                    FreeRefreshesCommands::Set { free_refreshes } => {
                        log::info!(
                            "setting the number of free refreshes in the market to {}",
                            free_refreshes,
                        );
                        market_configuration.free_refreshes = free_refreshes;
                        reply!(command, "Done.");
                    },
                }
            },
            MarketCommands::CanBeOpenedByPlayer { subcommand } => {
                match subcommand {
                    CanBeOpenedByPlayerCommands::Show => {
                        reply!(command, "{}", market_configuration.can_be_opened_by_player);
                    },
                    CanBeOpenedByPlayerCommands::Enable => {
                        if market_configuration.can_be_opened_by_player {
                            reply!(command, "Already enabled.");
                        } else {
                            log::info!("allowing the player to open the market");
                            market_configuration.can_be_opened_by_player = true;
                            reply!(command, "Done.");
                        }
                    },
                    CanBeOpenedByPlayerCommands::Disable => {
                        if !market_configuration.can_be_opened_by_player {
                            reply!(command, "Already disabled.");
                        } else {
                            log::info!("preventing the player from open the market");
                            market_configuration.can_be_opened_by_player = false;
                            reply!(command, "Done.");
                        }
                    },
                }
            },
        }
        reply!(command, "");
    }
}


/// Sets the balance and number of free refreshes during initialization.
pub fn set_balance_and_free_refreshes(
    args: Res<Args>,
    mut args_applied: Local<bool>,
    mut balance: ResMut<Balance>,
    mut market_configuration: ResMut<MarketConfiguration>,
) {
    if !(*args_applied) {
        if let Some(specified_balance) = args.start_in_game_balance {
            balance.0 = specified_balance;
        }
        if let Some(specified_free_refreshes) = args.start_in_game_free_refreshes {
            market_configuration.free_refreshes = specified_free_refreshes;
        }
        *args_applied = true;
    }
    log::info!(
        "player has {} and {} free refreshes",
        *balance,
        market_configuration.free_refreshes
    );
}


/// Gains balance when player earns experience.
pub fn gain_balance(
    mut event_reader: EventReader<ExperienceGainedEvent>,
    player_query: Query<&Player>,
    mut balance: ResMut<Balance>,
    experience_to_balance_ratio: Res<ExperienceToBalanceRatio>,
) {
    for event in event_reader.read() {
        if player_query.contains(event.entity) {
            let amount = Balance(event.experience.0 * experience_to_balance_ratio.0);
            balance.gain(
                amount,
                format!(
                    "gaining {} experience by {} (1.00 experience = {})",
                    event.experience,
                    event.by,
                    Balance(experience_to_balance_ratio.0),
                ),
            );
        }
    }
}


/// Processes acquirements from the market.
pub fn process_acquirements(
    mut market_state: ResMut<MarketState>,
    mut inventory: ResMut<Inventory>,
    item_registry: Res<ItemRegistry>,
) {
    while market_state.processed_acquirements < market_state.acquired_item_indices.len() {
        let index_of_item_to_acquire =
            market_state.acquired_item_indices[market_state.processed_acquirements];

        let item_id_to_acquire = &market_state.offered_item_ids[index_of_item_to_acquire];
        if let Some(item_to_acquire) = item_registry.find_item_by_id(item_id_to_acquire) {
            inventory.add(item_to_acquire.instantiate());
        }

        market_state.processed_acquirements += 1;
    }
}


/// Refreshes the items offered in the market.
pub fn refresh_market_automatically(world: &mut World) {
    let market_configuration = world.resource::<MarketConfiguration>();
    let number_of_items = market_configuration.number_of_items as usize;

    let mut market_state = world.resource_mut::<MarketState>();
    let mut previous_locked_item_count = 0;

    for item_index in 0..market_state.offered_item_ids.len() {
        let item_position = NonZeroUsize::new(item_index + 1).unwrap();
        if !market_state.is_acquired(item_position) {
            if market_state.is_locked(item_position) {
                previous_locked_item_count += 1;
            } else {
                market_state.locked_item_indices.push(item_index);
            }
        }
    }
    market_state.locked_item_indices.truncate(number_of_items);

    world.run_system_once(refresh_market);

    let mut market_state = world.resource_mut::<MarketState>();
    market_state.locked_item_indices.truncate(previous_locked_item_count);
}

/// Refreshes the items offered in the market.
pub fn refresh_market(
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    item_registry: Res<ItemRegistry>,
    market_configuration: Res<MarketConfiguration>,
    mut market_state: ResMut<MarketState>,
) {
    log::info!("refreshing the market to offer {} items", market_configuration.number_of_items);

    let mut new_offered_item_ids =
        Vec::with_capacity(market_configuration.number_of_items as usize);

    let mut seen_locked_item_indices = HashSet::new();
    for locked_item_index in market_state.locked_item_indices.iter().cloned() {
        let locked_item_position = NonZeroUsize::new(locked_item_index + 1).unwrap();
        if market_state.is_acquired(locked_item_position) {
            continue;
        }

        if let Some(previously_offered_item_id) =
            market_state.offered_item_ids.get(locked_item_index)
        {
            if seen_locked_item_indices.contains(&locked_item_index) {
                continue;
            }
            seen_locked_item_indices.insert(locked_item_index);

            if new_offered_item_ids.len() < (market_configuration.number_of_items as usize) {
                let price = item_registry
                    .find_item_by_id(previously_offered_item_id)
                    .map(|item| item.base_price)
                    .unwrap_or(Balance(f64::NAN));
                log::info!(
                    "re-offering locked \"{}\" at position {} in the market for {}",
                    previously_offered_item_id,
                    locked_item_position,
                    price,
                );
                new_offered_item_ids.push(previously_offered_item_id.clone());
            } else {
                log::error!(
                    "unable to re-offer locked \"{}\" at position {} in the market \
                    as the market already offers {} items",
                    previously_offered_item_id,
                    locked_item_position,
                    market_configuration.number_of_items,
                );
            }
        } else {
            log::warn!(
                "unable to re-offer the locked item at position {} in the market \
                    as it doesn't exist",
                locked_item_position,
            );
        }
    }

    let new_locked_item_indices = (0..new_offered_item_ids.len()).collect();

    if new_offered_item_ids.len() < (market_configuration.number_of_items as usize) {
        let mut commonness_of_items_that_can_be_offered = Vec::new();
        for entry in item_registry.iter() {
            let commonness = market_configuration.commonness_of(&entry.item);
            if commonness != 0 {
                commonness_of_items_that_can_be_offered.push((
                    entry.item.id(),
                    entry.item.base_price,
                    commonness,
                ));
            }
        }
        commonness_of_items_that_can_be_offered.sort_by(
            |(id1, _, commonness1), (id2, _, commonness)| {
                if commonness1 == commonness {
                    id1.cmp(id2)
                } else {
                    commonness1.cmp(commonness).reverse()
                }
            },
        );

        let number_of_items_to_offer_randomly =
            (market_configuration.number_of_items as usize) - new_offered_item_ids.len();
        if commonness_of_items_that_can_be_offered.is_empty() {
            log::error!(
                "unable to randomly select {} more item{} to offer in the market \
                as no item is eligible to be offered in the market",
                number_of_items_to_offer_randomly,
                if number_of_items_to_offer_randomly == 1 { "" } else { "s" },
            );
        } else {
            let total_commonness = commonness_of_items_that_can_be_offered
                .iter()
                .map(|(_, _, commonness)| commonness)
                .sum::<u64>();

            let mut probability_table = Table::new();
            probability_table.add_row(row![c -> "Item", c -> "Chance", c -> "Probability"]);
            for (id, _, commonness) in commonness_of_items_that_can_be_offered.iter() {
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
                "{}item{} to offer will be selected randomly with these probabilities:\n{}",
                if new_offered_item_ids.is_empty() {
                    "".to_owned()
                } else {
                    format!("{} more ", number_of_items_to_offer_randomly)
                },
                if number_of_items_to_offer_randomly == 1 { "" } else { "s" },
                probability_table.trim_end(),
            );

            while new_offered_item_ids.len() != (market_configuration.number_of_items as usize) {
                match commonness_of_items_that_can_be_offered
                    .choose_weighted(rng.deref_mut(), |(_, _, commonness)| *commonness)
                {
                    Ok((id, price, commonness)) => {
                        log::info!(
                            "offering randomly selected \"{}\" \
                            with {:.6}% probability ({} / {}) for {}",
                            id,
                            ((*commonness as f64) / (total_commonness as f64)) * 100.00,
                            commonness,
                            total_commonness,
                            price,
                        );
                        new_offered_item_ids.push((*id).clone())
                    },
                    Err(error) => {
                        log::error!(
                            "unable to choose a random item to offer in the market ({})",
                            error,
                        );
                        break;
                    },
                }
            }
        }
    }

    market_state.offered_item_ids = new_offered_item_ids;
    market_state.locked_item_indices = new_locked_item_indices;

    market_state.acquired_item_indices.clear();
    market_state.processed_acquirements = 0;

    log::info!("market is refreshed");
}


/// Opens the market.
pub fn open_market(
    game_action_state_query: Query<&ActionState<GameAction>, With<Player>>,
    selected_game_mode_id: Res<SelectedGameModeId>,
    market_configuration: Res<MarketConfiguration>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(game_action_state) = game_action_state_query.get_single() {
        if game_action_state.just_pressed(&GameAction::OpenMarket) {
            if market_configuration.can_be_opened_by_player {
                log::info!("opening the market");
                game_state_stack.push(GameState::Market);
                next_game_state.set(GameState::Transition);
            } else {
                log::warn!(
                    "unable to open the market: cannot be opened by the player in {:?} mode",
                    selected_game_mode_id.0,
                );
            }
        }
    }
}


/// Resets the market.
pub fn reset_market(mut commands: Commands) {
    commands.insert_resource(Balance::default());
    commands.insert_resource(ExperienceToBalanceRatio::default());
    commands.insert_resource(MarketConfiguration::default());
    commands.insert_resource(MarketState::default());
}
