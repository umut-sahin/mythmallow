use crate::prelude::*;


/// Resource for the current wave.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
pub struct CurrentWave(pub u8);

impl Default for CurrentWave {
    fn default() -> CurrentWave {
        CurrentWave(1)
    }
}


/// Resource for the remaining time to complete the current wave.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
pub struct WaveTimer(pub Timer);
