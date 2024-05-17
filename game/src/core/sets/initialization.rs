use crate::prelude::*;

/// Systems to run when initializing the game.
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum InitializationSystems {
    First,
    GameMode,
    Player,
    Market,
    Leveling,
    Inventory,
    Last,
    Done,
}

impl InitializationSystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        InitializationSystems::iter().zip(InitializationSystems::iter().skip(1)).for_each(
            |(set1, set2)| {
                app.configure_sets(OnEnter(GameState::Initialization), set1.before(set2));
                app.configure_sets(OnExit(GameState::Initialization), set1.before(set2));
            },
        );
    }
}
