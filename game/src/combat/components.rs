use crate::prelude::*;


/// Tag component for attacks.
#[derive(Component, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub enum Attack {
    Contact,
    Thrust { direction: Vec2, range: Range, duration: Duration, started: bool },
}


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


/// Component for the originator of entities.
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct Originator(pub Entity);

impl From<Entity> for Originator {
    fn from(entity: Entity) -> Originator {
        Originator(entity)
    }
}


/// Tag component for projectiles.
#[derive(Component, Debug, Reflect)]
pub struct Projectile;


/// Bundle for projectiles.
#[derive(Bundle, TypedBuilder)]
pub struct ProjectileBundle {
    #[builder(setter(into))]
    pub originator: Originator,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub collider: Collider,
    pub position: Position,
    pub velocity: LinearVelocity,
    pub damage: Damage,
}

impl ProjectileBundle {
    /// Spawns the projectile.
    fn spawn<'c>(
        self,
        commands: &'c mut Commands,
        layers: CollisionLayers,
        additional_components: impl Bundle,
    ) -> EntityCommands<'c> {
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
    pub fn spawn_toward_player<'c>(self, commands: &'c mut Commands) -> EntityCommands<'c> {
        let layers = CollisionLayers::new(
            [Layer::Projectile, Layer::DamagePlayer],
            [Layer::MapBound, Layer::MapObstacle, Layer::PlayerHitBox],
        );
        self.spawn(commands, layers, (DamagePlayerOnContact, Attack::Contact))
    }

    /// Spawns the projectile toward enemies.
    pub fn spawn_toward_enemies<'c>(self, commands: &'c mut Commands) -> EntityCommands<'c> {
        let layers = CollisionLayers::new(
            [Layer::Projectile, Layer::DamageEnemies],
            [Layer::MapBound, Layer::MapObstacle, Layer::EnemyHitBox],
        );
        self.spawn(commands, layers, (DamageEnemiesOnContact, Attack::Contact))
    }
}


/// Component for remaining health.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct RemainingHealth(pub f32);
