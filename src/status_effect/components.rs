use crate::prelude::*;


/// Dynamic status effect component for cooldowns.
#[derive(Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Cooldown<T: Send + Sync + 'static> {
    /// Timer to track how much of the cooldown is remaining.
    pub timer: Timer,

    #[reflect(ignore)]
    phantom: PhantomData<T>,
}

impl<T: Send + Sync + 'static> Cooldown<T> {
    /// Creates a cooldown.
    pub fn new(timer: Timer) -> Cooldown<T> {
        Cooldown { timer, phantom: PhantomData }
    }
}

impl<T: Send + Sync + 'static> Debug for Cooldown<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s", self.timer.remaining_secs())
    }
}
