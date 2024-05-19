use crate::prelude::*;

/// Systems to run when restarting the game.
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum RestartSystems {
    First,
    Hud,
    Market,
    Enemy,
    Camera,
    GameMode,
    Inventory,
    Map,
    Player,
    Leveling,
    Last,
    Done,
}

impl RestartSystems {
    /// Configure the system set.
    pub fn configure(app: &mut App) {
        RestartSystems::iter().zip(RestartSystems::iter().skip(1)).for_each(|(set1, set2)| {
            app.configure_sets(OnEnter(GameState::Restart), set1.before(set2));
            app.configure_sets(OnExit(GameState::Restart), set1.before(set2));
        });
    }
}
