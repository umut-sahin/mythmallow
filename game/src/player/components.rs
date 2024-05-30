use crate::prelude::*;


/// Tag component for the player.
#[derive(Component, Debug, Default, Reflect)]
pub struct Player;


/// Tag component for the hit box of the player.
#[derive(Component, Debug, Default, Reflect)]
pub struct PlayerHitBox;

impl PlayerHitBox {
    /// Creates a bundle for the player hit box.
    pub fn bundle(collider: Collider) -> impl Bundle {
        (
            // Tags
            Name::new("Hit Box"),
            PlayerHitBox,
            // Physics
            collider,
            CollisionLayers::new([Layer::PlayerHitBox], [Layer::DamagePlayer]),
            Sensor,
        )
    }
}


/// Tag component for the pickup area of the player.
#[derive(Component, Debug, Default, Reflect)]
pub struct PlayerPickupArea;

impl PlayerPickupArea {
    /// Creates a bundle for the player pickup area.
    pub fn bundle(collider: Collider) -> impl Bundle {
        (
            // Tags
            Name::new("Pickup Area"),
            PlayerPickupArea,
            // Physics
            collider,
            CollisionLayers::new([Layer::PlayerPickupArea], [Layer::ExperiencePoint]),
            Sensor,
        )
    }
}


/// Tag component for entities that apply damage to the player on contact.
#[derive(Component, Debug, Default, Reflect)]
pub struct DamagePlayerOnContact;


/// Bundle for players.
#[derive(Bundle, TypedBuilder)]
pub struct PlayerBundle<P: Component + IPlayer> {
    pub player: P,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    #[builder(setter(transform =
        |input_map: InputMap<GameAction>| {
            InputManagerBundle::<GameAction> { action_state: ActionState::default(), input_map }
        }
    ))]
    pub input: InputManagerBundle<GameAction>,
}

impl<P: Component + IPlayer> PlayerBundle<P> {
    /// Spawns the player.
    pub fn spawn<'c>(self, commands: &'c mut Commands) -> EntityCommands<'c> {
        let name = format!("Player [{}]", self.player.name());
        let health = self.player.health();
        let pickup_range = self.player.pickup_range();
        let speed = self.player.speed();
        let speed_multiplier = self.player.speed_multiplier();
        let collider = self.player.collider();

        let mut player = commands.spawn((
            // Tags
            Name::new(name),
            Player,
            // Player
            self,
            health,
            pickup_range,
            speed,
            speed_multiplier,
            // Combat
            RemainingHealth(*health),
            // Leveling
            Level::default(),
            Experience::default(),
            // Physics
            (
                RigidBody::Dynamic,
                LinearVelocity::ZERO,
                Restitution::PERFECTLY_INELASTIC,
                LockedAxes::ROTATION_LOCKED,
                collider.clone(),
                CollisionLayers::new([Layer::Player], [Layer::MapBound, Layer::ExperiencePoint]),
            ),
        ));

        player.with_children(|parent| {
            parent.spawn(PlayerHitBox::bundle(collider));
            parent.spawn(PlayerPickupArea::bundle(Collider::circle(*pickup_range)));
        });

        player
    }
}
