use crate::{
    constants::*,
    prelude::*,
    styles,
};


/// Initializes the game mode.
pub fn initialize(
    mut commands: Commands,
    hud_query: Query<Entity, With<Hud>>,
    asset_server: Res<AssetServer>,
) {
    let wave_durations = WaveDurations::new(WAVES);
    let current_wave = CurrentWave::default();
    let level_up_rewards = LevelUpRewards::default();

    if let Ok(hud) = hud_query.get_single() {
        let wave_duration =
            wave_durations.get(current_wave.index()).copied().unwrap_or(Duration::ZERO);

        commands.entity(hud).with_children(|parent| {
            parent
                .spawn((
                    Name::new("Current Wave"),
                    CurrentWaveContainer,
                    NodeBundle { style: styles::current_wave_container(), ..default() },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Text"),
                        CurrentWaveText,
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("Wave {}", current_wave.0),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: CURRENT_WAVE_TEXT_FONT_SIZE,
                                        color: CURRENT_WAVE_TEXT_COLOR,
                                    },
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    Name::new("Remaining Seconds"),
                    RemainingSecondsContainer,
                    NodeBundle { style: styles::remaining_seconds_container(), ..default() },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Text"),
                        RemainingSecondsText,
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("{:.0}", wave_duration.as_secs_f32().ceil()),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: REMAINING_SECONDS_TEXT_FONT_SIZE,
                                        color: REMAINING_SECONDS_TEXT_COLOR,
                                    },
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
    }

    commands.insert_resource(wave_durations);
    commands.insert_resource(current_wave);
    commands.insert_resource(level_up_rewards);
}

/// Selects the wave from the arguments of the survival game mode.
pub fn select_wave_when_starting_in_game(
    args: Res<Args>,
    survival_mode_args: Res<SurvivalModeArgs>,
    mut current_wave: ResMut<CurrentWave>,
) {
    if args.start_in_game {
        if let Some(wave) = &survival_mode_args.start_in_game_waves {
            *current_wave = CurrentWave(*wave);
        }
    }
}


/// Loads the current wave.
pub fn load(
    mut commands: Commands,
    mut remaining_seconds_text_query: Query<&mut Text, With<RemainingSecondsText>>,
    current_wave: Res<CurrentWave>,
    wave_durations: Res<WaveDurations>,
) {
    log::info!("starting wave {}", current_wave.0);

    let wave_duration = wave_durations.get(current_wave.index()).copied().unwrap_or(Duration::ZERO);
    log::info!("wave duration: {:?}", wave_duration);
    commands.insert_resource(WaveTimer::new(wave_duration));

    if let Ok(mut remaining_seconds_text) = remaining_seconds_text_query.get_single_mut() {
        remaining_seconds_text.sections[0].value =
            format!("{:.0}", wave_duration.as_secs_f32().ceil());
    }
}

/// Spawns the map.
pub fn spawn_map(mut commands: Commands) {
    commands.insert_resource(MAP_BOUNDS);
    commands.spawn((Name::new("Map"), Map, SpatialBundle::default())).with_children(|parent| {
        // Spawn horizontal lines.
        for i in 0..=GRID_SIZE {
            parent.spawn((
                Name::new(format!("Horizontal Line {}", i + 1)),
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        0.00,
                        (((GRID_SIZE as f32) / 2.00) - (i as f32)) * GRID_SPACING,
                        Depth::Map.z(),
                    )),
                    sprite: Sprite {
                        color: GRID_COLOR,
                        custom_size: Some(Vec2::new(GRID_SIZE as f32 * GRID_SPACING, GRID_WIDTH)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
        // Spawn vertical lines.
        for i in 0..=GRID_SIZE {
            parent.spawn((
                Name::new(format!("Vertical Line {}", i + 1)),
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        ((i as f32) - ((GRID_SIZE as f32) / 2.00)) * GRID_SPACING,
                        0.00,
                        Depth::Map.z(),
                    )),
                    sprite: Sprite {
                        color: GRID_COLOR,
                        custom_size: Some(Vec2::new(GRID_WIDTH, GRID_SIZE as f32 * GRID_SPACING)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    });
}


/// Ticks wave timer and wins the current wave when wave timer is finished.
pub fn tick(
    mut remaining_seconds_text_query: Query<&mut Text, With<RemainingSecondsText>>,
    time: Res<Time>,
    mut wave_timer: ResMut<WaveTimer>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    wave_timer.tick(time.delta());
    if let Ok(mut remaining_seconds_text) = remaining_seconds_text_query.get_single_mut() {
        remaining_seconds_text.sections[0].value =
            format!("{:.0}", wave_timer.remaining_secs().ceil());
    }

    if wave_timer.just_finished() {
        game_state_stack.transition(GameState::Won);
        next_game_state.set(GameState::Transition);
    }
}

/// Processes player level change.
pub fn level_change(
    mut commands: Commands,
    mut player_query: Query<
        (&Level, &mut Health, &mut RemainingHealth),
        (With<Player>, Changed<Level>),
    >,
    mut level_up_rewards: ResMut<LevelUpRewards>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok((player_level, mut player_health, mut player_remaining_health)) =
        player_query.get_single_mut()
    {
        {
            let expected_bonus_health = (player_level.get() as f32) - 1.00;
            if level_up_rewards.health.0 != expected_bonus_health {
                let difference = expected_bonus_health - level_up_rewards.health.0;

                if difference > 0.00 {
                    log::info!(
                        "increasing the player health by {} for leveling up to level {}",
                        difference,
                        player_level.get(),
                    );
                } else {
                    log::info!(
                        "decreasing the player health by {} for leveling down to level {}",
                        difference.abs(),
                        player_level.get(),
                    );
                }

                player_health.0 += difference;
                player_remaining_health.0 += difference;

                level_up_rewards.health.0 += difference;
            }
        }

        {
            let expected_number_of_perks = (player_level.get() as usize) - 1;
            if level_up_rewards.perks.len() > expected_number_of_perks {
                let mut level_to_lose = level_up_rewards.perks.len();
                for perk in level_up_rewards.perks[expected_number_of_perks..].iter_mut().rev() {
                    commands.run_system_with_input(
                        registered_systems.perk.lose_perk,
                        (
                            perk.clone(),
                            ObtainLosePerkReason::LevelingDown {
                                to: Level::new(level_to_lose as u16),
                            },
                        ),
                    );
                    level_to_lose -= 1;
                }
            }
            level_up_rewards.perks.truncate(expected_number_of_perks);
        }
    }
}

/// Processes obtaining a new perk.
pub fn obtain_perk(
    mut level_up_rewards: ResMut<LevelUpRewards>,
    mut perk_obtained_event_reader: EventReader<PerkObtainedEvent>,
) {
    for event in perk_obtained_event_reader.read() {
        if matches!(event.reason, ObtainLosePerkReason::LevelingUp { .. }) {
            level_up_rewards.perks.push(event.perk.clone());
        }
    }
}


/// Wins the current wave.
pub fn win(
    mut commands: Commands,
    mut current_wave_text_query: Query<&mut Text, With<CurrentWaveText>>,
    mut player_query: Query<(&Level, &mut RemainingHealth, &Health), With<Player>>,
    mut current_wave: ResMut<CurrentWave>,
    level_up_rewards: Res<LevelUpRewards>,
    mut market_configuration: ResMut<MarketConfiguration>,
    mut level_up_screen_configuration: ResMut<LevelUpScreenConfiguration>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    if current_wave.is_last() {
        log::info!("game won!");
        commands.insert_resource(GameResult::Won);

        game_state_stack.transition(GameState::Over);
        next_game_state.set(GameState::Transition);
    } else {
        log::info!("wave {} won", current_wave.0);

        let refresh_cost =
            MarketRefreshCost::exponential(Balance(current_wave.get() as f64), 1.50, None);
        log::info!("setting the refresh cost model of the market to {}", refresh_cost);
        market_configuration.refresh_cost = refresh_cost;

        commands.run_system(registered_systems.market.refresh_market);

        let reroll_cost =
            LevelUpScreenRerollCost::exponential(Balance(current_wave.get() as f64), 1.50, None);
        log::info!("setting the reroll cost model of the level up screen to {}", reroll_cost);
        level_up_screen_configuration.reroll_cost = reroll_cost;

        game_state_stack.pop();
        game_state_stack.push(GameState::Loading);
        game_state_stack.push(GameState::Market);

        if let Ok((player_level, mut player_remaining_health, player_health)) =
            player_query.get_single_mut()
        {
            log::info!("resetting player health to {}", player_health.0);
            player_remaining_health.0 = player_health.0;

            let reward_count = ((player_level.get() as usize) - 1) - level_up_rewards.perks.len();
            for _ in 0..reward_count {
                game_state_stack.push(GameState::LevelUpScreen);
            }

            if reward_count > 0 {
                commands.insert_resource(LevelUpScreenReason::LevelingUp {
                    to: Level::new(level_up_rewards.perks.len() as u16 + 2),
                });
            }
        }

        next_game_state.set(GameState::Transition);

        current_wave.increment();

        if let Ok(mut current_wave_text) = current_wave_text_query.get_single_mut() {
            current_wave_text.sections[0].value = format!("Wave {}", current_wave.0);
        }
    }
}


/// Unloads the current wave.
pub fn unload(mut commands: Commands) {
    commands.remove_resource::<WaveTimer>();
}


/// Deinitializes the game mode.
pub fn deinitialize(mut commands: Commands) {
    commands.remove_resource::<CurrentWave>();
}
