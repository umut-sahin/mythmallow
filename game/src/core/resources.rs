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


/// State stack for the game state.
#[derive(Debug, Default, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct GameStateStack(pub Vec<GameState>);

impl GameStateStack {
    /// Transitions to a new game state.
    pub fn transition(&mut self, state: GameState) {
        self.0.pop();
        self.0.push(state);
    }
}


/// Result of the game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect, Resource)]
pub enum GameResult {
    Won,
    Lost,
}
