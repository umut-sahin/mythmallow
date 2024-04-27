use crate::prelude::*;

/// Systems to run when loading the game.
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum LoadingSystems {
    GameMode,
    Player,
    Leveling,
    Map,
    Camera,
    Enemy,
    Done,
}

impl LoadingSystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        for (current, next) in LoadingSystems::iter().zip(LoadingSystems::iter().skip(1)) {
            app.configure_sets(OnEnter(GameState::Loading), current.before(next));
            app.configure_sets(OnExit(GameState::Loading), current.before(next));
        }
    }
}
