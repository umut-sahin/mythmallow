use crate::prelude::*;

/// Systems to run when restarting the game.
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum RestartSystems {
    Enemy,
    Camera,
    GameMode,
    Inventory,
    Map,
    Player,
    Done,
}

impl RestartSystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        let done = RestartSystems::iter().next_back().unwrap();
        for set in RestartSystems::iter() {
            if set != done {
                app.configure_sets(OnEnter(GameState::Restart), set.before(done));
                app.configure_sets(OnExit(GameState::Restart), set.before(done));
            }
        }
    }
}
