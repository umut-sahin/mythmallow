use crate::prelude::*;


/// Database of registered systems.
#[derive(Debug, Resource)]
pub struct RegisteredSystems {
    pub leveling: RegisteredLevelingSystems,
    pub market: RegisteredMarketSystems,
}

impl RegisteredSystems {
    /// Creates the database.
    pub fn new(app: &mut App) -> RegisteredSystems {
        RegisteredSystems {
            leveling: RegisteredLevelingSystems::new(app),
            market: RegisteredMarketSystems::new(app),
        }
    }
}


/// Result of the game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect, Resource)]
pub enum GameResult {
    Won,
    Lost,
}
