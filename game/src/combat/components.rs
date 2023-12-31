use crate::prelude::*;


/// Tag component for attacks.
#[derive(Component, Debug, Reflect)]
pub struct Attack;


/// Component for cooldown of applying damage.
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct DamageCooldown {
    pub duration: Duration,
}

impl DamageCooldown {
    /// Creates a new damage cooldown.
    pub fn new(duration: Duration) -> DamageCooldown {
        DamageCooldown { duration }
    }
}


/// Tag component for projectiles.
#[derive(Component, Debug, Reflect)]
pub struct Projectile;


/// Bundle for projectiles.
#[derive(Bundle)]
pub struct ProjectileBundle {
    // Tags
    pub tag: Projectile,
    // Properties
    pub damage: Damage,
    // Physics
    pub body: RigidBody,
    pub position: Position,
    pub velocity: LinearVelocity,
    pub collider: Collider,
    pub layers: CollisionLayers,
    // Texture
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}


/// Component for remaining health.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct RemainingHealth(pub f32);
