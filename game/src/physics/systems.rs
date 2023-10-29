use crate::prelude::*;


/// Resumes physics simulation.
pub fn resume_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.unpause();
}

/// Pauses physics simulation.
pub fn pause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.pause();
}
