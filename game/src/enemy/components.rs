use crate::prelude::*;


/// Tag component for enemies.
#[derive(Component, Debug, Default, Reflect)]
pub struct Enemy;


/// Tag component for hit boxes of enemies.
#[derive(Component, Debug, Default, Reflect)]
pub struct EnemyHitBox;

impl EnemyHitBox {
    pub fn bundle<E: Munchie>(enemy: &E) -> impl Bundle {
        (
            // Tags
            Name::new("Hit Box"),
            EnemyHitBox,
            // Physics
            enemy.collider(),
            CollisionLayers::new([Layer::EnemyHitBox], [Layer::DamageEnemies]),
            Sensor,
        )
    }
}


/// Tag component for entities that apply damage to enemies on contact.
#[derive(Component, Debug, Default, Reflect)]
pub struct DamageEnemiesOnContact;


/// Bundle for enemies.
#[derive(Bundle)]
pub struct EnemyBundle {
    // Tags
    pub name: Name,
    pub tag: Enemy,
    // Properties
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
