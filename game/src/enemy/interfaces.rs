use crate::prelude::*;

/// Interface for enemy packs.
pub trait MunchiePack: Any + Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the enemy pack.
    fn id(&self) -> SmolStr;
    /// Gets the name of the enemy pack.
    fn name(&self) -> SmolStr;

    /// Gets the spawn pattern of the enemy pack,
    #[allow(unused_variables)]
    fn spawn_pattern(&self, world: &World) -> Option<EnemySpawnPattern> {
        None
    }
}

/// Interface for enemies.
pub trait Munchie: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the enemy.
    fn id(&self) -> SmolStr;
    /// Gets the name of the enemy.
    fn name(&self) -> SmolStr;

    /// Gets the collider of the enemy.
    fn collider(&self) -> Collider;

    /// Spawns the enemy.
    fn spawn(&self, world: &mut World, position: Position);
}
