use crate::prelude::*;


/// Run condition for game mode.
pub fn in_game_mode<T: IGameMode>(mode: Option<Res<GameMode<T>>>) -> bool {
    mode.is_some()
}
