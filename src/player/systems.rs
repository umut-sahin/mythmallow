use crate::{
    player::constants::*,
    prelude::*,
};

/// Spawns the player.
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_action_input_map: Res<InputMap<GameAction>>,
) {
    commands
        .spawn((
            // Tags
            Name::new("Player"),
            Player,
            // Properties
            Damage(INITIAL_PLAYER_DAMAGE),
            Health(INITIAL_PLAYER_HEALTH),
            Speed(INITIAL_PLAYER_SPEED),
            // Combat
            RemainingHealth(INITIAL_PLAYER_HEALTH),
            // Physics
            RigidBody::Dynamic,
            Position(Vector::new(0.0, 0.0)),
            Collider::ball(PLAYER_SIZE),
            LinearVelocity(Vector::new(0.0, 0.0)),
            CollisionLayers::new([Layer::Player], [Layer::Player]),
            // Texture
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_translation(Vec3::new(0.00, 0.00, 2.00)),
                ..default()
            },
            // Input
            InputManagerBundle::<GameAction> {
                action_state: ActionState::default(),
                input_map: game_action_input_map.clone(),
            },
        ))
        .with_children(|child_builder| {
            child_builder.spawn((Collider::ball(PLAYER_SIZE), PlayerHitbox, Sensor));
        });
}

/// Despawns the player.
pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).despawn_recursive();
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


/// Opens pause menu on pause action.
pub fn pause_the_game(
    game_action_state_query: Query<&ActionState<GameAction>, With<Player>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(game_action_state) = game_action_state_query.get_single() {
        if game_action_state.just_pressed(GameAction::Pause) {
            next_game_state.set(GameState::Paused);
        }
    }
}
