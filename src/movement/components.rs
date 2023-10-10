use crate::prelude::*;


/// Dynamic movement component for dashing.
#[derive(Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Dashing {
    /// Timer to track how much of the dash is remaining.
    pub timer: Timer,
}

impl Debug for Dashing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s", self.timer.remaining_secs())
    }
}
