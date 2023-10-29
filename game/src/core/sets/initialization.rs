use crate::prelude::*;

/// Systems to run when initializing the game.
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum InitializationSystems {
    GameMode,
    Player,
    Done,
}

impl InitializationSystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        let done = InitializationSystems::iter().next_back().unwrap();
        for set in InitializationSystems::iter() {
            if set != done {
                app.configure_sets(OnEnter(GameState::Initialization), set.before(done));
                app.configure_sets(OnExit(GameState::Initialization), set.before(done));
            }
        }
    }
}
