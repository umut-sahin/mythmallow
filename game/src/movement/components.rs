use crate::prelude::*;


/// Component for attraction.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
#[component(storage = "SparseSet")]
pub struct AttractedTo(pub Entity);


/// Component for the speed of attraction.
#[derive(Clone, Component, Debug, Reflect)]
pub enum AttractionSpeed {
    Constant(Speed),
    Accelerating {
        min_speed: Speed,
        acceleration_per_second: Speed,
        current_speed: Speed,
        max_speed: Speed,
    },
}


/// Component for the ideal distance to the attracted object.
#[derive(Clone, Copy, Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct IdealAttractionDistance(pub f32);


/// Component for the slowdown when going backwards towards the ideal distance.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct SlowdownOfGoingBackwardsDuringAttraction(pub f32);

impl Default for SlowdownOfGoingBackwardsDuringAttraction {
    fn default() -> Self {
        SlowdownOfGoingBackwardsDuringAttraction(1.00)
    }
}


/// Component for dashing.
#[derive(Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Dashing {
    /// Timer to track how much time is left until dashing is over.
    pub timer: Timer,
}

impl Debug for Dashing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s", self.timer.remaining_secs())
    }
}
