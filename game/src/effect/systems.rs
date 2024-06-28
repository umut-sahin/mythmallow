use crate::prelude::*;


/// Pauses effects.
pub fn pause_effects(mut effect_simulation_time: ResMut<Time<EffectSimulation>>) {
    effect_simulation_time.pause();
}

/// Resumes effects.
pub fn resume_effects(mut effect_simulation_time: ResMut<Time<EffectSimulation>>) {
    effect_simulation_time.unpause();
}
