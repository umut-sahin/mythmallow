use mythmallow::prelude::*;


/// Resource for the current wave.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct CurrentWave(pub u8);

impl Default for CurrentWave {
    fn default() -> CurrentWave {
        CurrentWave(1)
    }
}


/// Resource for the remaining time to complete the current wave.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct WaveTimer(pub Timer);

impl WaveTimer {
    /// Creates a new wave timer.
    pub fn new(duration: Duration) -> WaveTimer {
        WaveTimer(Timer::new(duration, TimerMode::Once))
    }
}

impl Default for WaveTimer {
    fn default() -> WaveTimer {
        WaveTimer(Timer::new(Duration::from_secs(60), TimerMode::Once))
    }
}
