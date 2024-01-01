use crate::prelude::*;


/// Size of the player.
pub const PLAYER_SIZE: f32 = 20.00;


/// Base health of players.
pub const BASE_PLAYER_HEALTH: f32 = 10.00;

/// Base speed of players.
pub const BASE_PLAYER_SPEED: f32 = 200.00;


/// Base duration of dashing of players.
pub const BASE_DASH_DURATION: Duration = Duration::from_millis(75);

/// Base cooldown of dashing of players.
pub const BASE_DASH_COOLDOWN: Duration = Duration::from_millis(1000);
