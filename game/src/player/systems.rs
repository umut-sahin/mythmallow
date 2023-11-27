use crate::{
    player::constants::*,
    prelude::*,
};


/// Spawns the player.
pub fn spawn_player(world: &mut World) {
    let player_registry = PLAYER_REGISTRY.lock().unwrap();
    let selection = world.resource::<SelectedPlayerIndex>();
    player_registry[*selection].spawn(world);
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
pub fn reset_player_position(
    mut player_query: Query<(&mut Position, &mut Transform), With<Player>>,
) {
    if let Ok((mut player_position, mut player_transform)) = player_query.get_single_mut() {
        player_position.x = 0.00;
        player_position.y = 0.00;
        player_transform.translation.x = 0.00;
        player_transform.translation.y = 0.00;
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
        (&ActionState<GameAction>, &Speed, &mut LinearVelocity),
        (With<Player>, Without<Dashing>),
    >,
) {
    let (action_state, speed, mut velocity) = match player_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let mut change = Vec2::ZERO;

    if action_state.pressed(GameAction::MoveUp) {
        change.y += 1.0;
    }
    if action_state.pressed(GameAction::MoveLeft) {
        change.x -= 1.0;
    }
    if action_state.pressed(GameAction::MoveDown) {
        change.y -= 1.0;
    }
    if action_state.pressed(GameAction::MoveRight) {
        change.x += 1.0;
    }

    velocity.0 = if change == Vec2::ZERO { Vec2::ZERO } else { change.normalize() * speed.0 }
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

    if action_state.just_pressed(GameAction::Dash) {
        if velocity.0 == Vec2::ZERO {
            return;
        }
        commands.entity(entity).insert((
            Dashing { timer: Timer::new(INITIAL_DASH_DURATION, TimerMode::Once) },
            Cooldown::<Dashing>::new(Timer::new(INITIAL_DASH_COOLDOWN, TimerMode::Once)),
        ));
    }
}


/// Pauses the game.
pub fn pause(
    game_action_state_query: Query<&ActionState<GameAction>, With<Player>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(game_action_state) = game_action_state_query.get_single() {
        if game_action_state.just_pressed(GameAction::Pause) {
            next_game_state.set(GameState::Paused);
        }
    }
}


/// Clears player selection.
pub fn clear_player_selection(mut commands: Commands) {
    commands.remove_resource::<SelectedPlayerIndex>();
    commands.remove_resource::<SelectedPlayer>();
}
