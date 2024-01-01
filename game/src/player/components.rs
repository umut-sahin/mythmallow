use crate::prelude::*;


/// Tag component for the player.
#[derive(Component, Debug, Reflect)]
pub struct Player;


/// Tag component for the hit box of the player.
#[derive(Component, Debug, Reflect)]
pub struct PlayerHitBox;


/// Tag component for entities that apply damage to the player on contact.
#[derive(Component, Debug, Reflect)]
pub struct DamagePlayerOnContact;


/// Bundle for players.
#[derive(Bundle)]
pub struct PlayerBundle {
    // Tags
    pub name: Name,
    pub tag: Player,
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
    pub velocity: LinearVelocity,
    pub layers: CollisionLayers,
    pub axes: LockedAxes,
    // Texture
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    // Input
    pub input: InputManagerBundle<GameAction>,
}
