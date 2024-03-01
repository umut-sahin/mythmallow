use crate::prelude::*;


/// Tag component for enemies.
#[derive(Component, Debug, Default, Reflect)]
pub struct Enemy;


/// Tag component for hit boxes of enemies.
#[derive(Component, Debug, Default, Reflect)]
pub struct EnemyHitBox;

impl EnemyHitBox {
    pub fn bundle(collider: Collider) -> impl Bundle {
        (
            // Tags
            Name::new("Hit Box"),
            EnemyHitBox,
            // Physics
            collider,
            CollisionLayers::new([Layer::EnemyHitBox], [Layer::DamageEnemies]),
            Sensor,
        )
    }
}


/// Tag component for entities that apply damage to enemies on contact.
#[derive(Component, Debug, Default, Reflect)]
pub struct DamageEnemiesOnContact;


/// Bundle for enemies.
#[derive(Bundle, TypedBuilder)]
pub struct EnemyBundle<E: Component + IEnemy> {
    pub enemy: E,
    pub position: Position,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl<E: Component + IEnemy> EnemyBundle<E> {
    /// Spawns the enemy.
    pub fn spawn<'c>(
        self,
        commands: &'c mut Commands,
        counter: &mut EnemyCounter,
    ) -> EntityCommands<'c> {
        counter.increment();

        let name = self.enemy.name();
        let contact_damage = self.enemy.contact_damage();
        let health = self.enemy.health();
        let speed = self.enemy.speed();
        let collider = self.enemy.collider();

        let mut collision_groups = LayerMask::from([Layer::Enemy]);
        let mut collision_masks = LayerMask::from([Layer::MapBound, Layer::Enemy]);

        if contact_damage.is_some() {
            collision_groups.add([Layer::DamagePlayer]);
            collision_masks.add([Layer::PlayerHitBox]);
        }

        let collision_layers = CollisionLayers::new(collision_groups, collision_masks);

        let mut enemy = commands.spawn((
            // Tags
            Name::new(format!("Enemy {} [{}]", counter.get(), name)),
            Enemy,
            // Properties
            self,
            health,
            speed,
            // Combat
            RemainingHealth(*health),
            // Physics
            RigidBody::Dynamic,
            LinearVelocity::ZERO,
            Restitution::PERFECTLY_INELASTIC,
            LockedAxes::ROTATION_LOCKED,
            collider.clone(),
            collision_layers,
        ));

        enemy.with_children(|parent| {
            parent.spawn(EnemyHitBox::bundle(collider));
        });

        if let Some((damage, cooldown)) = contact_damage {
            enemy.insert((DamagePlayerOnContact, damage, cooldown));
        }

        enemy
    }
}


/// Component for the ideal distance to the player.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct IdealDistanceToPlayer(pub f32);
