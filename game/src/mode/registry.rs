use crate::prelude::*;

/// Registry for game modes.
pub static GAME_MODE_REGISTRY: Mutex<GameModeRegistry> = Mutex::new(GameModeRegistry::new());

/// Container for game mode registry.
#[derive(Default, Deref, DerefMut, Resource)]
pub struct GameModeRegistry(pub Vec<Arc<dyn Mode>>);

impl GameModeRegistry {
    /// Creates a new game mode registry.
    pub const fn new() -> GameModeRegistry {
        GameModeRegistry(Vec::new())
    }
}

impl GameModeRegistry {
    /// Registers a game mode to game mode registry.
    pub fn register(&mut self, game_mode: impl Mode) {
        if self.iter().any(|existing_entry| existing_entry.id() == game_mode.id()) {
            log::warn!("tried to register {:?} to game mode registry again", game_mode.name());
        } else {
            log::info!("registered {:?} to game mode registry", game_mode.name());
            self.push(Arc::new(game_mode));
        }
    }
}

impl Index<GameModeIndex> for GameModeRegistry {
    type Output = Arc<dyn Mode>;

    fn index(&self, index: GameModeIndex) -> &Arc<dyn Mode> {
        &self.deref()[index.0]
    }
}
