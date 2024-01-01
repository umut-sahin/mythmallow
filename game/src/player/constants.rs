use crate::prelude::*;


/// Size of the player.
pub const PLAYER_SIZE: f32 = 20.00;


/// Initial health of the player.
pub const INITIAL_PLAYER_HEALTH: f32 = 10.00;

/// Initial speed of the player.
pub const INITIAL_PLAYER_SPEED: f32 = 200.00;


/// Initial duration of dashing of the player.
pub const INITIAL_DASH_DURATION: Duration = Duration::from_millis(75);

/// Initial cooldown of dashing of the player.
pub const INITIAL_DASH_COOLDOWN: Duration = Duration::from_millis(1000);
