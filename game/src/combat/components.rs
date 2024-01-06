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
#[derive(Bundle, TypedBuilder)]
pub struct ProjectileBundle {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub collider: Collider,
    pub position: Position,
    pub velocity: LinearVelocity,
    pub damage: Damage,
}

impl ProjectileBundle {
    /// Spawns the projectile.
    fn spawn<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
        layers: CollisionLayers,
        additional_components: impl Bundle,
    ) -> EntityCommands<'w, 's, 'a> {
        commands.spawn((
            // Tags
            Name::new("Projectile"),
            Projectile,
            // Projectile
            self,
            additional_components,
            // Physics
            RigidBody::Dynamic,
            layers,
        ))
    }

    /// Spawns the projectile toward the player.
    pub fn spawn_toward_player<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
    ) -> EntityCommands<'w, 's, 'a> {
        let layers = CollisionLayers::new(
            [Layer::Projectile, Layer::DamagePlayer],
            [Layer::MapBound, Layer::MapObstacle, Layer::PlayerHitBox],
        );
        self.spawn(commands, layers, DamagePlayerOnContact)
    }

    /// Spawns the projectile toward enemies.
    pub fn spawn_toward_enemies<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
    ) -> EntityCommands<'w, 's, 'a> {
        let layers = CollisionLayers::new(
            [Layer::Projectile, Layer::DamageEnemies],
            [Layer::MapBound, Layer::MapObstacle, Layer::EnemyHitBox],
        );
        self.spawn(commands, layers, DamageEnemiesOnContact)
    }
}


/// Component for remaining health.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct RemainingHealth(pub f32);
