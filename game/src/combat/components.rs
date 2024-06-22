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


/// Component for the name of the originator of entities.
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct OriginatorName(pub Name);

impl From<&str> for OriginatorName {
    fn from(name: &str) -> OriginatorName {
        OriginatorName(name.into())
    }
}

impl From<String> for OriginatorName {
    fn from(name: String) -> OriginatorName {
        OriginatorName(name.into())
    }
}

impl From<SmolStr> for OriginatorName {
    fn from(name: SmolStr) -> OriginatorName {
        OriginatorName(name.as_str().into())
    }
}

impl From<Name> for OriginatorName {
    fn from(name: Name) -> OriginatorName {
        OriginatorName(name)
    }
}

impl From<&Name> for OriginatorName {
    fn from(name: &Name) -> OriginatorName {
        OriginatorName(name.clone())
    }
}


/// Tag component for projectiles.
#[derive(Component, Debug, Reflect)]
pub struct Projectile;


/// Bundle for projectiles.
#[derive(Bundle, TypedBuilder)]
pub struct ProjectileBundle {
    #[builder(setter(into))]
    pub originator: OriginatorName,
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
        self.spawn(commands, layers, DamagePlayerOnContact)
    }

    /// Spawns the projectile toward enemies.
    pub fn spawn_toward_enemies<'c>(self, commands: &'c mut Commands) -> EntityCommands<'c> {
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
