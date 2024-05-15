use crate::{
    market::commands::*,
    prelude::*,
};


/// Applies the market console commands.
pub fn apply_market_command(
    mut commands: Commands,
    mut market_configuration: ResMut<MarketConfiguration>,
    mut market_state: ResMut<MarketState>,
    refresh_market_system_id: Res<RefreshMarketSystemId>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
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
                if market_state.offered_items.is_empty() {
                    reply!(command, "Market is empty.");
                } else {
                    for (index, item) in market_state.offered_items.iter().enumerate() {
                        let position = NonZeroUsize::new(index + 1).unwrap();
                        reply!(
                            command,
                            "{}) {} {}",
                            position,
                            item.id(),
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
                commands.run_system(refresh_market_system_id.0);
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
            MarketCommands::Open => {
                match game_state.get() {
                    GameState::Playing | GameState::Paused => {
                        log::info!("opening the market");
                        next_game_state.set(GameState::Market);
                        reply!(command, "Opened.");
                    },
                    GameState::Market => {
                        reply!(command, "Failed to open the market as it's already opened.");
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
                        next_game_state.set(GameState::Playing);
                        reply!(command, "Closed.");
                    },
                    GameState::Playing | GameState::Paused => {
                        reply!(command, "Failed to close the market as it's already closed.");
                    },
                    _ => {
                        reply!(command, "How did you time this, seriously?");
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
                        reply!(command, "{} $", market_configuration.refresh_cost(&market_state));
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
                                            MarketRefreshCost::constant(Experience(cost));
                                    },
                                    RefreshCostModelSetCommands::Linear {
                                        base,
                                        increase,
                                        step,
                                        max,
                                    } => {
                                        market_configuration.refresh_cost =
                                            MarketRefreshCost::linear(
                                                Experience(base),
                                                Experience(increase),
                                                max.map(Experience),
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
                                                Experience(base),
                                                factor,
                                                max.map(Experience),
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
        }
        reply!(command, "");
    }
}


/// Processes acquirements from the market.
pub fn process_acquirements(
    mut market_state: ResMut<MarketState>,
    mut inventory: ResMut<Inventory>,
) {
    while market_state.processed_acquirements < market_state.acquired_item_indices.len() {
        let index_of_item_to_acquire =
            market_state.acquired_item_indices[market_state.processed_acquirements];

        let item_to_acquire = &market_state.offered_items[index_of_item_to_acquire];
        inventory.add(item_to_acquire.instantiate());

        market_state.processed_acquirements += 1;
    }
}


/// Refreshes the items offered in the market.
pub fn refresh_market_automatically(world: &mut World) {
    let market_configuration = world.resource::<MarketConfiguration>();
    let number_of_items = market_configuration.number_of_items as usize;

    let mut market_state = world.resource_mut::<MarketState>();
    let mut previous_locked_item_count = 0;

    for item_index in 0..market_state.offered_items.len() {
        let item_position = NonZeroUsize::new(item_index + 1).unwrap();
        if market_state.is_locked(item_position) {
            if !market_state.is_acquired(item_position) {
                previous_locked_item_count += 1;
            }
        } else {
            market_state.locked_item_indices.push(item_index);
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

    let mut new_offered_items = Vec::with_capacity(market_configuration.number_of_items as usize);

    let mut seen_locked_item_indices = HashSet::new();
    for locked_item_index in market_state.locked_item_indices.iter().cloned() {
        let locked_item_position = NonZeroUsize::new(locked_item_index + 1).unwrap();
        if market_state.is_acquired(locked_item_position) {
            continue;
        }

        if let Some(previously_offered_item) = market_state.offered_items.get(locked_item_index) {
            if seen_locked_item_indices.contains(&locked_item_index) {
                continue;
            }
            seen_locked_item_indices.insert(locked_item_index);

            if new_offered_items.len() < (market_configuration.number_of_items as usize) {
                log::info!(
                    "re-offering locked \"{}\" at position {} in the market",
                    previously_offered_item.id(),
                    locked_item_position,
                );
                new_offered_items.push(previously_offered_item.clone());
            } else {
                log::error!(
                    "unable to re-offer locked \"{}\" at position {} in the market \
                    as the market already offers {} items",
                    previously_offered_item.id(),
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

    let new_locked_item_indices = (0..new_offered_items.len()).collect();

    if new_offered_items.len() < (market_configuration.number_of_items as usize) {
        let mut commonness_of_items_that_can_be_offered = Vec::new();
        for entry in item_registry.iter() {
            let commonness = market_configuration.commonness_of(&entry.item);
            if commonness != 0 {
                commonness_of_items_that_can_be_offered.push((
                    entry.item.id(),
                    &entry.item,
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
            (market_configuration.number_of_items as usize) - new_offered_items.len();
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
                .map(|(_, _, weight)| weight)
                .sum::<u64>();

            let mut probability_table = Table::new();
            probability_table.add_row(row![c -> "Item", c -> "Chance", c -> "Probability"]);
            for (id, _, weight) in commonness_of_items_that_can_be_offered.iter() {
                probability_table.add_row(row![
                    l -> id,
                    r -> format!("({} / {})", weight, total_commonness),
                    r -> format!("{:.6}%", ((*weight as f64) / (total_commonness as f64)) * 100.00)
                ]);
            }
            let probability_table = probability_table.to_string();

            log::info!(
                "{}item{} to offer will be selected randomly with these probabilities:\n{}",
                if new_offered_items.is_empty() {
                    "".to_owned()
                } else {
                    format!("{} more ", number_of_items_to_offer_randomly)
                },
                if number_of_items_to_offer_randomly == 1 { "" } else { "s" },
                probability_table.trim_end(),
            );

            while new_offered_items.len() != (market_configuration.number_of_items as usize) {
                match commonness_of_items_that_can_be_offered
                    .choose_weighted(rng.deref_mut(), |(_, _, weight)| *weight)
                {
                    Ok((id, new_offered_item, weight)) => {
                        log::info!(
                            "offering randomly selected \"{}\" with {:.6}% probability ({} / {})",
                            id,
                            ((*weight as f64) / (total_commonness as f64)) * 100.00,
                            weight,
                            total_commonness,
                        );
                        new_offered_items.push((*new_offered_item).clone())
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

    market_state.offered_items = new_offered_items;
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
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(game_action_state) = game_action_state_query.get_single() {
        if game_action_state.just_pressed(&GameAction::OpenMarket) {
            if market_configuration.can_be_opened_by_player {
                log::info!("opening the market");
                next_game_state.set(GameState::Market);
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
    commands.insert_resource(MarketConfiguration::default());
    commands.insert_resource(MarketSpending::default());
    commands.insert_resource(MarketState::default());
}
