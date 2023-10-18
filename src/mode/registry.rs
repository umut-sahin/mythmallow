use crate::prelude::*;

/// Registry for game modes.
pub static GAME_MODE_REGISTRY: Mutex<GameModeRegistry> = Mutex::new(GameModeRegistry::new());


/// Container for game mode registry.
#[derive(Default, Resource)]
pub struct GameModeRegistry {
    registered_game_mode_type_ids: Vec<TypeId>,
    registered_game_modes: Vec<Box<dyn Mode>>,
}

impl GameModeRegistry {
    /// Creates a new game mode registry.
    pub const fn new() -> GameModeRegistry {
        GameModeRegistry {
            registered_game_mode_type_ids: Vec::new(),
            registered_game_modes: Vec::new(),
        }
    }
}

impl GameModeRegistry {
    /// Registers game mode to registry.
    pub fn register<M: Mode>(&mut self, mode: M) {
        let type_id = TypeId::of::<M>();
        if !self.registered_game_mode_type_ids.contains(&type_id) {
            self.registered_game_mode_type_ids.push(type_id);
            self.registered_game_modes.push(Box::new(mode));
        } else {
            log::warn!(
                "{} is tried to be registered to game mode registry again",
                std::any::type_name::<M>(),
            );
        }
    }
}

impl Deref for GameModeRegistry {
    type Target = Vec<Box<dyn Mode>>;

    fn deref(&self) -> &Vec<Box<dyn Mode>> {
        &self.registered_game_modes
    }
}
