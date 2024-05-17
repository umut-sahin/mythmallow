use crate::{
    leveling::commands::*,
    prelude::*,
};


/// Applies the inventory console commands.
pub fn apply_experience_command(
    player_query: Query<(Entity, &Experience), With<Player>>,
    mut experience_gained_event_writer: EventWriter<ExperienceGainedEvent>,
    mut command: ConsoleCommand<ExperienceCommand>,
) {
    if let Some(Ok(ExperienceCommand { subcommand })) = command.take() {
        let (player_entity, player_experience) = if let Ok(query_result) = player_query.get_single()
        {
            query_result
        } else {
            reply!(command, "Not available outside the game.");
            reply!(command, "");
            return;
        };

        match subcommand {
            ExperienceCommands::Show => {
                reply!(command, "{}", *player_experience);
            },
            ExperienceCommands::Add { experience } => {
                experience_gained_event_writer.send(ExperienceGainedEvent {
                    entity: player_entity,
                    experience: Experience(experience),
                    by: "cheating :)".to_owned(),
                });
                reply!(command, "Added.");
            },
        }

        reply!(command, "");
    }
}

/// Applies the level console commands.
pub fn apply_level_command(
    mut commands: Commands,
    mut player_query: Query<&Level, With<Player>>,
    registered_systems: Res<RegisteredSystems>,
    mut command: ConsoleCommand<LevelCommand>,
) {
    if let Some(Ok(LevelCommand { subcommand })) = command.take() {
        let player_level = if let Ok(query_result) = player_query.get_single_mut() {
            query_result
        } else {
            reply!(command, "Not available outside the game.");
            reply!(command, "");
            return;
        };

        match subcommand {
            LevelCommands::Show => {
                reply!(command, "{}", player_level.get());
            },
            LevelCommands::Set { level } => {
                commands.run_system_with_input(registered_systems.leveling.set_level, Level(level));
                reply!(command, "Set.");
            },
        }

        reply!(command, "");
    }
}


/// Sets the level of the player.
pub fn set_level(In(mut level): In<Level>, world: &mut World) {
    let player_level_structure = world.resource::<PlayerLevelStructure>().clone();

    let max_level = player_level_structure.max_level.unwrap_or(Level(NonZeroU16::MAX));
    if level.0 > max_level.0 {
        level = max_level;
    }

    let experience_required_to_get_to_current_level = if level.get() == 1 {
        Experience(0.00)
    } else {
        *(player_level_structure.required_experience_calculator)(
            world,
            Level(NonZeroU16::new(level.get() - 1).unwrap()),
        )
    };
    let experience_required_to_level_up = if level.0 != max_level.0 {
        *(player_level_structure.required_experience_calculator)(world, level)
    } else {
        Experience(f64::INFINITY)
    };

    let (mut player_level, mut player_experience) = world
        .query_filtered::<(&mut Level, &mut Experience), With<Player>>()
        .get_single_mut(world)
        .unwrap();
    let old_level = *player_level;

    *player_level = level;
    *player_experience = experience_required_to_get_to_current_level;

    log::info!("setting player to level {}", player_level.get());
    log::info!("setting player experience to {}", *player_experience);
    log::info!("requiring {} experience for the next level", experience_required_to_level_up);

    world.insert_resource(ExperienceRequiredToGetToCurrentLevel(
        experience_required_to_get_to_current_level,
    ));
    world.insert_resource(ExperienceRequiredToLevelUp(experience_required_to_level_up));

    if level.0 > old_level.0 {
        let player_entity = world.query_filtered::<Entity, With<Player>>().single(world);
        let mut leveled_up_events = world.resource_mut::<Events<LeveledUpEvent>>();

        let number_of_level_ups = level.get() - old_level.get();
        for i in 1..=number_of_level_ups {
            leveled_up_events.send(LeveledUpEvent {
                entity: player_entity,
                new_level: Level(old_level.checked_add(i).unwrap()),
            });
        }
    }
}


/// Initializes the player level structure.
pub fn initialize_player_level_structure(world: &mut World) {
    let game_mode_registry = world.resource::<GameModeRegistry>();
    let selected_game_mode_index = world.resource::<SelectedGameModeIndex>();
    let player_level_structure =
        game_mode_registry[*selected_game_mode_index].player_level_structure();

    let args = world.resource::<Args>();

    let mut specified_level = args.start_in_game_level.map(Level).unwrap_or_default();
    let specified_experience = args.start_in_game_experience.map(Experience).unwrap_or_default();

    let max_level = player_level_structure.max_level.unwrap_or(Level(NonZeroU16::MAX));
    if specified_level.0 > max_level.0 {
        specified_level = max_level;
    }

    let experience_required_to_get_to_current_level = if specified_level.get() == 1 {
        Experience(0.00)
    } else {
        *(player_level_structure.required_experience_calculator)(
            world,
            Level(NonZeroU16::new(specified_level.get() - 1).unwrap()),
        )
    };
    let mut experience_required_to_level_up = if specified_level.0 != max_level.0 {
        *(player_level_structure.required_experience_calculator)(world, specified_level)
    } else {
        Experience(f64::INFINITY)
    };

    let mut set_level = specified_level;
    let set_experience =
        Experience(specified_experience.max(*experience_required_to_get_to_current_level));

    while set_level.0 < max_level.0 && set_experience.0 >= experience_required_to_level_up.0 {
        set_level = Level(set_level.0.checked_add(1).unwrap());
        experience_required_to_level_up = if set_level.0 != max_level.0 {
            *(player_level_structure.required_experience_calculator)(world, set_level)
        } else {
            Experience(f64::INFINITY)
        };
    }

    let (mut player_level, mut player_experience) = world
        .query_filtered::<(&mut Level, &mut Experience), With<Player>>()
        .get_single_mut(world)
        .unwrap();

    *player_level = set_level;
    *player_experience = set_experience;

    log::info!("player is level {} with {} experience", set_level.0, set_experience);
    log::info!("requiring {} experience for the next level", experience_required_to_level_up);

    world.insert_resource(ExperienceRequiredToGetToCurrentLevel(
        experience_required_to_get_to_current_level,
    ));
    world.insert_resource(ExperienceRequiredToLevelUp(experience_required_to_level_up));
    world.insert_resource(player_level_structure);

    let player_entity = world.query_filtered::<Entity, With<Player>>().single(world);
    let mut leveled_up_events = world.resource_mut::<Events<LeveledUpEvent>>();
    for player_new_level in 2..=set_level.get() {
        leveled_up_events.send(LeveledUpEvent {
            entity: player_entity,
            new_level: Level(NonZeroU16::new(player_new_level).unwrap()),
        });
    }
}

/// Initializes the experience point counter.
pub fn initialize_experience_point_counter(mut commands: Commands) {
    commands.insert_resource(ExperiencePointCounter::default());
}


/// Attracts the experience points inside player pickup area towards the player.
pub fn attract_experience_points(
    mut commands: Commands,
    experience_point_query: Query<Entity, With<ExperiencePoint>>,
    player_pickup_area_query: Query<&Parent, With<PlayerPickupArea>>,
    player_query: Query<Entity, With<Player>>,
    mut collision_started_event_reader: EventReader<CollisionStarted>,
) {
    let mut attract_if_applicable = |player_pickup_area_entity, experience_point_entity| {
        let experience_point_entity = match experience_point_query.get(experience_point_entity) {
            Ok(query_result) => query_result,
            Err(_) => return,
        };
        let player_entity = match player_pickup_area_query.get(player_pickup_area_entity) {
            Ok(parent) => player_query.get(parent.get()).unwrap(),
            Err(_) => return,
        };
        commands.entity(experience_point_entity).insert(AttractedTo(player_entity));
    };

    for CollisionStarted(entity1, entity2) in collision_started_event_reader.read().cloned() {
        attract_if_applicable(entity1, entity2);
        attract_if_applicable(entity2, entity1);
    }
}

/// Collects experience points.
pub fn collect_experience_points(
    mut commands: Commands,
    player_query: Query<&Player>,
    experience_point_query: Query<(&Name, &Experience), (With<ExperiencePoint>, Without<Player>)>,
    mut collision_started_event_reader: EventReader<CollisionStarted>,
    mut experience_gained_event_writer: EventWriter<ExperienceGainedEvent>,
) {
    let mut collect_if_applicable = |player_entity, experience_point_entity| {
        if player_query.get(player_entity).is_err() {
            return;
        }
        let (experience_point_name, experience_reward) =
            match experience_point_query.get(experience_point_entity) {
                Ok(query_result) => query_result,
                Err(_) => return,
            };
        experience_gained_event_writer.send(ExperienceGainedEvent {
            entity: player_entity,
            experience: *experience_reward,
            by: format!("collecting \"{}\"", experience_point_name),
        });
        commands.entity(experience_point_entity).despawn_recursive();
    };

    for CollisionStarted(entity1, entity2) in collision_started_event_reader.read().cloned() {
        collect_if_applicable(entity1, entity2);
        collect_if_applicable(entity2, entity1);
    }
}

/// Gains experience for the player.
pub fn gain_player_experience(
    mut player_query: Query<&mut Experience, With<Player>>,
    mut experience_gained_event_reader: EventReader<ExperienceGainedEvent>,
) {
    for experience_changed_event in experience_gained_event_reader.read() {
        if let Ok(mut player_experience) = player_query.get_mut(experience_changed_event.entity) {
            **player_experience += *experience_changed_event.experience;
            log::info!(
                "player gained {} experience by {}",
                experience_changed_event.experience,
                experience_changed_event.by,
            );
        }
    }
}

/// Levels up the player.
pub fn level_player_up(world: &mut World) {
    loop {
        let mut system_state: SystemState<(
            Query<(Entity, &Experience, &mut Level), With<Player>>,
            Res<ExperienceRequiredToLevelUp>,
            Res<PlayerLevelStructure>,
            EventWriter<LeveledUpEvent>,
        )> = SystemState::new(world);

        let (
            mut player_query,
            experience_required_to_level_up,
            player_level_structure,
            mut leveled_up_event_writer,
        ) = system_state.get_mut(world);

        if let Ok((player_entity, player_experience, mut player_level)) =
            player_query.get_single_mut()
        {
            log::info!("trying to level up the player");

            if player_experience.0 < *experience_required_to_level_up.0 {
                log::info!(
                    "player required {} experience to level up but has {} experience",
                    experience_required_to_level_up.0,
                    player_experience,
                );
                break;
            }

            let max_level = player_level_structure.max_level.unwrap_or(Level::new(u16::MAX));
            if player_level.0 >= max_level.0 {
                return;
            }

            log::info!(
                "player required {} experience to level up and has {} experience",
                experience_required_to_level_up.0,
                player_experience,
            );

            let new_player_level = Level(player_level.checked_add(1).unwrap());
            log::info!("player leveled up to level {}", new_player_level.get());

            *player_level = new_player_level;
            leveled_up_event_writer
                .send(LeveledUpEvent { entity: player_entity, new_level: new_player_level });

            let new_experience_required_to_get_to_current_level =
                ExperienceRequiredToGetToCurrentLevel(experience_required_to_level_up.0);

            let new_experience_required_to_level_up = if new_player_level.0 != max_level.0 {
                (player_level_structure.required_experience_calculator)(world, new_player_level)
            } else {
                ExperienceRequiredToLevelUp(Experience(f64::INFINITY))
            };

            log::info!(
                "requiring {} experience for the next level",
                new_experience_required_to_level_up.0,
            );

            world.insert_resource(new_experience_required_to_get_to_current_level);
            world.insert_resource(new_experience_required_to_level_up);
        }
    }
}


/// Despawns experience points.
pub fn despawn_experience_points(
    mut commands: Commands,
    experience_point_query: Query<Entity, With<ExperiencePoint>>,
) {
    for entity in &experience_point_query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Clears the experience point counter.
pub fn clear_experience_point_counter(mut commands: Commands) {
    commands.remove_resource::<ExperiencePointCounter>();
}

/// Clears the player level structure.
pub fn clear_player_level_structure(mut commands: Commands) {
    commands.remove_resource::<ExperienceRequiredToGetToCurrentLevel>();
    commands.remove_resource::<ExperienceRequiredToLevelUp>();
    commands.remove_resource::<PlayerLevelStructure>();
}
