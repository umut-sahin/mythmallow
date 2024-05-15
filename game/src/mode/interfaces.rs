use crate::prelude::*;


/// Interface for game modes.
pub trait IGameMode: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the game mode.
    fn id(&self) -> SmolStr;
    /// Gets the name of the game mode.
    fn name(&self) -> SmolStr;

    /// Gets whether the market can be opened by the player in the game mode.
    fn market_can_be_opened_by_player(&self) -> bool;
    /// Gets the default enemy spawn pattern of the game mode.
    fn default_enemy_spawn_pattern(&self, world: &World) -> EnemySpawnPattern;
    /// Gets the player level structure of the game mode.
    fn player_level_structure(&self) -> PlayerLevelStructure;

    /// Initializes the game mode.
    fn initialize(&self, world: &mut World);
    /// Deinitializes the game mode.
    fn deinitialize(&self, world: &mut World);
}
