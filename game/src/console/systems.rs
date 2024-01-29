use crate::prelude::*;


/// (Un)pauses physics time depending on the console state.
pub fn control_physics_time(
    mut physics_time: ResMut<Time<Physics>>,
    console_state: Res<ConsoleState>,
    mut previously_paused: Local<bool>,
) {
    if console_state.open {
        *previously_paused = physics_time.is_paused();
        physics_time.pause();
    } else if !(*previously_paused) {
        physics_time.unpause();
    }
}
