use crate::prelude::*;


/// Container for the game mode registry.
#[derive(Debug, Default, Deref, Resource)]
pub struct GameModeRegistry(Vec<GameModeRegistryEntry>);

impl GameModeRegistry {
    /// Registers a game mode to the game mode registry.
    pub fn register(&mut self, game_mode: impl IGameMode) -> &mut RegisteredGameMode {
        let game_mode_id = game_mode.id();
        let game_mode_index = match self
            .iter()
            .position(|entry| entry.game_mode.id() == game_mode_id)
        {
            Some(index) => {
                log::warn!("tried to register {:?} to the game mode registry again", game_mode_id);
                index
            },
            None => {
                log::info!("registered {:?} to the game mode registry", game_mode_id);
                let index = self.len();
                self.0.push(GameModeRegistryEntry::new(game_mode));
                index
            },
        };
        &mut self.0[game_mode_index].game_mode
    }
}

impl GameModeRegistry {
    /// Gets the number of game modes in the game mode registry.
    pub fn number_of_game_modes(&self) -> usize {
        self.len()
    }
}

impl Index<SelectedGameModeIndex> for GameModeRegistry {
    type Output = GameModeRegistryEntry;

    fn index(&self, game_mode_index: SelectedGameModeIndex) -> &GameModeRegistryEntry {
        &self.0[*game_mode_index]
    }
}


/// Container for the entries of the game mode registry.
#[derive(Debug)]
pub struct GameModeRegistryEntry {
    pub game_mode: RegisteredGameMode,
}

impl GameModeRegistryEntry {
    /// Creates a new game mode registry entry.
    pub fn new(game_mode: impl IGameMode) -> GameModeRegistryEntry {
        GameModeRegistryEntry { game_mode: RegisteredGameMode::new(game_mode) }
    }
}

impl Deref for GameModeRegistryEntry {
    type Target = RegisteredGameMode;

    fn deref(&self) -> &RegisteredGameMode {
        &self.game_mode
    }
}


/// Container for registered game modes.
#[derive(Debug)]
pub struct RegisteredGameMode {
    pub game_mode: Arc<dyn IGameMode>,
}

impl RegisteredGameMode {
    /// Creates a new registered game mode.
    pub fn new(game_mode: impl IGameMode) -> RegisteredGameMode {
        RegisteredGameMode { game_mode: Arc::new(game_mode) }
    }
}

impl Deref for RegisteredGameMode {
    type Target = Arc<dyn IGameMode>;

    fn deref(&self) -> &Arc<dyn IGameMode> {
        &self.game_mode
    }
}
