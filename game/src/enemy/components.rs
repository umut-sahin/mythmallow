use crate::prelude::*;


/// Tag component for enemies.
#[derive(Component, Debug, Reflect)]
pub struct Enemy;


/// Bundle for enemies.
#[derive(Bundle)]
pub struct EnemyBundle {
    // Tags
    pub name: Name,
    pub tag: Enemy,
    // Properties
    pub damage: Damage,
    pub health: Health,
    pub speed: Speed,
    // Combat
    pub remaining_health: RemainingHealth,
    // Physics
    pub body: RigidBody,
    pub restitution: Restitution,
    pub position: Position,
    pub collider: Collider,
    pub layers: CollisionLayers,
    // Texture
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}
