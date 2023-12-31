use crate::prelude::*;


/// Component for cooldowns.
#[derive(Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Cooldown<T: Send + Sync + 'static> {
    /// Timer to track how much time is left until cooldown is over.
    pub timer: Timer,

    #[reflect(ignore)]
    phantom: PhantomData<T>,
}

impl<T: Send + Sync + 'static> Cooldown<T> {
    /// Creates a cooldown.
    pub fn new(duration: Duration) -> Cooldown<T> {
        Cooldown { timer: Timer::new(duration, TimerMode::Once), phantom: PhantomData }
    }
}

impl<T: Send + Sync + 'static> Debug for Cooldown<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s", self.timer.remaining_secs())
    }
}
