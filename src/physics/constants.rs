/// Rate of physics updates per second.
pub const TICK_RATE: f32 = 60.00;

/// Amount of seconds between physics updates.
pub const DELTA_TIME: f32 = 1.00 / TICK_RATE;


/// Amount of substeps per physics update.
pub const SUBSTEPS: u32 = 10;

/// Amount of seconds between substeps.
pub const SUBSTEP_DELTA_TIME: f32 = DELTA_TIME / (SUBSTEPS as f32);
