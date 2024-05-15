use crate::prelude::*;


/// (Un)pauses physics time depending on the console state.
pub fn control_physics_time(
    mut physics_time: ResMut<Time<Physics>>,
    console_state: Res<ConsoleState>,
    game_state: Res<State<GameState>>,
) {
    if console_state.open {
        physics_time.pause();
    } else if game_state.physics_enabled() {
        physics_time.unpause();
    }
}
