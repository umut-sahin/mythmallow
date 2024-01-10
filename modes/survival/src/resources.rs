use {
    crate::constants::*,
    mythmallow::prelude::*,
};


/// Resource for the current wave.
#[derive(Clone, Copy, Debug, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct CurrentWave(pub NonZeroU8);

impl CurrentWave {
    pub fn is_last(&self) -> bool {
        self.get() == WAVES
    }
}

impl CurrentWave {
    pub fn increment(&mut self) {
        if self.is_last() {
            panic!("tried to go past the last wave");
        }
        self.0 = self.checked_add(1).unwrap();
    }
}

impl Default for CurrentWave {
    fn default() -> CurrentWave {
        CurrentWave(NonZeroU8::new(1).unwrap())
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


/// Resource for the duration of waves.
#[derive(Debug, Default, Deref, Reflect, Resource)]
#[reflect(Resource)]
pub struct WaveDurations(Vec<Duration>);

impl WaveDurations {
    pub fn new(waves: u8) -> WaveDurations {
        let mut result = Vec::with_capacity(waves as usize);
        for wave in 1..=waves {
            match wave {
                1 => {
                    result.push(Duration::from_secs_f32(10.00));
                },
                2 => {
                    result.push(Duration::from_secs_f32(15.00));
                },
                3 => {
                    result.push(Duration::from_secs_f32(20.00));
                },
                _ => {
                    result.push(Duration::from_secs_f32(30.00));
                },
            }
        }
        WaveDurations(result)
    }
}

impl Index<CurrentWave> for WaveDurations {
    type Output = Duration;

    fn index(&self, current_wave: CurrentWave) -> &Duration {
        &self.0[(current_wave.get() - 1) as usize]
    }
}
