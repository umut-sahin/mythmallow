use crate::prelude::*;


/// Physics layers to differentiate collisions.
#[derive(Debug, PhysicsLayer, Reflect)]
pub enum Layer {
    /// Layer for the invisible bounds of the map.
    MapBound,
    /// Layer for the obstacles in the map.
    MapObstacle,

    /// Layer for the player.
    Player,
    /// Layer for the enemies.
    Enemy,

    /// Layer for the hit box of the player.
    PlayerHitBox,
    /// Layer for damaging the player.
    DamagePlayer,

    /// Layer for the hit box of the enemies.
    EnemyHitBox,
    /// Layer for damaging the enemies.
    DamageEnemies,

    /// Layer for projectiles.
    Projectile,

    /// Layer for the pickup area of the player.
    PlayerPickupArea,
    /// Layer for experience points.
    ExperiencePoint,
}
