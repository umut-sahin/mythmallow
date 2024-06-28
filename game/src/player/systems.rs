use crate::{
    player::{
        commands::*,
        constants::*,
    },
    prelude::*,
};


/// Applies the player console commands.
pub fn apply_player_command(
    mut command: ConsoleCommand<PlayerCommand>,
    mut god_mode: ResMut<GodMode>,
) {
    if let Some(Ok(PlayerCommand { subcommand })) = command.take() {
        match subcommand {
            PlayerCommands::GodMode { subcommand } => {
                match subcommand {
                    GodModeCommands::Status => {
                        let status = if god_mode.is_enabled { "Enabled" } else { "Disabled" };
                        reply!(command, "{}.", status);
                    },
                    GodModeCommands::Enable => {
                        if god_mode.is_enabled {
                            reply!(command, "Already enabled.");
                        } else {
                            god_mode.is_enabled = true;
                            reply!(command, "Enabled.");
                        }
                    },
                    GodModeCommands::Disable => {
                        if god_mode.is_enabled {
                            god_mode.is_enabled = false;
                            reply!(command, "Disabled.");
                        } else {
                            reply!(command, "Already disabled.");
                        }
                    },
                }
            },
        }
        reply!(command, "");
    }
}


/// Spawns the player.
pub fn spawn_player(world: &mut World) {
    let player_registry = world.resource::<PlayerRegistry>();
    let selected_mythology_index = world.resource::<SelectedMythologyIndex>();
    let selected_player_index = world.resource::<SelectedPlayerIndex>();
    player_registry[*selected_mythology_index][*selected_player_index].clone().spawn(world);
}

/// Despawns the player.
pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


/// Makes the player hidden.
pub fn turn_player_visibility_off(mut player_query: Query<&mut Visibility, With<Player>>) {
    if let Ok(mut player_visibility) = player_query.get_single_mut() {
        *player_visibility = Visibility::Hidden;
    }
}

/// Resets the player position.
pub fn reset_player_position(mut player_query: Query<&mut Position, With<Player>>) {
    if let Ok(mut player_position) = player_query.get_single_mut() {
        player_position.x = 0.00;
        player_position.y = 0.00;
    }
}

/// Makes the player visible.
pub fn turn_player_visibility_on(mut player_query: Query<&mut Visibility, With<Player>>) {
    if let Ok(mut player_visibility) = player_query.get_single_mut() {
        *player_visibility = Visibility::Visible;
    }
}


/// Moves the player.
pub fn movement(
    mut player_query: Query<
        (
            &ActionState<GameAction>,
            &Speed,
            &SpeedMultiplier,
            &mut LastWalkingParticlePosition,
            &Position,
            &mut LinearVelocity,
        ),
        (With<Player>, Without<Dashing>),
    >,
    mut walking_effect_query: Query<(&mut EffectSpawner, &mut Transform), With<WalkingParticles>>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    let (
        action_state,
        speed,
        speed_multiplier,
        mut last_walking_particle_position,
        position,
        mut velocity,
    ) = match player_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let mut change = Vec2::ZERO;

    if action_state.pressed(&GameAction::MoveUp) {
        change.y += 1.0;
    }
    if action_state.pressed(&GameAction::MoveLeft) {
        change.x -= 1.0;
    }
    if action_state.pressed(&GameAction::MoveDown) {
        change.y -= 1.0;
    }
    if action_state.pressed(&GameAction::MoveRight) {
        change.x += 1.0;
    }

    if change == Vec2::ZERO {
        velocity.0 = Vec2::ZERO;

        last_walking_particle_position.0 = position.0;
    } else {
        velocity.0 = change.normalize() * (speed.0 * speed_multiplier.0);

        let particle_gap = position.distance(last_walking_particle_position.0);

        if particle_gap >= WALKING_EFFECT_GAP {
            let (mut spawner, mut effect_transform) = match walking_effect_query.get_single_mut() {
                Ok(query_result) => query_result,
                Err(_) => return,
            };

            last_walking_particle_position.0 = position.0;

            effect_transform.translation = Vec3::new(
                position.x + rng.gen_range(-WALKING_EFFECT_SPREAD..WALKING_EFFECT_SPREAD),
                position.y + rng.gen_range(-WALKING_EFFECT_SPREAD..WALKING_EFFECT_SPREAD),
                0.0,
            );

            spawner.reset();
        }
    }
}

/// Activates dashing for the player.
pub fn dash(
    mut commands: Commands,
    player_query: Query<
        (Entity, &ActionState<GameAction>, &LinearVelocity),
        (With<Player>, Without<Cooldown<Dashing>>),
    >,
) {
    let (entity, action_state, velocity) = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if action_state.just_pressed(&GameAction::Dash) {
        if velocity.0 == Vec2::ZERO {
            return;
        }
        commands.entity(entity).insert((
            Dashing { timer: Timer::new(BASE_DASH_DURATION, TimerMode::Once) },
            Cooldown::<Dashing>::new(BASE_DASH_COOLDOWN),
        ));
    }
}


/// Pauses the game.
pub fn pause(
    game_action_state_query: Query<&ActionState<GameAction>, With<Player>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(game_action_state) = game_action_state_query.get_single() {
        if game_action_state.just_pressed(&GameAction::Pause) {
            game_state_stack.push(GameState::Paused);
            next_game_state.set(GameState::Transition);
        }
    }
}


/// Clears player selection.
pub fn clear_player_selection(mut commands: Commands) {
    commands.remove_resource::<SelectedMythologyIndex>();
    commands.remove_resource::<SelectedPlayerIndex>();
}
