use crate::{
    leveling::constants::*,
    prelude::*,
};


/// Component for level.
#[derive(Clone, Component, Copy, Debug, Deref, DerefMut, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Level(pub NonZeroU16);

impl Level {
    /// Creates a new level.
    ///
    /// # Panics
    ///
    /// - Panics if `level` is zero.
    pub fn new(level: u16) -> Level {
        Level(NonZeroU16::new(level).expect("expected level to be strictly positive"))
    }
}

impl Default for Level {
    fn default() -> Level {
        Level(NonZeroU16::new(1).unwrap())
    }
}


/// Component for experience.
#[derive(Clone, Copy, Component, Debug, Default, Deref, DerefMut, PartialOrd, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Experience(pub f64);

impl Experience {
    /// Zero experience.
    pub const ZERO: Experience = Experience(0.00);

    /// One experience.
    pub const ONE: Experience = Experience(1.00);
}

impl Display for Experience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_nan() {
            return write!(f, "?");
        }
        if self.is_infinite() {
            write!(f, "{}∞", if self.is_sign_positive() { "" } else { "-" })
        } else {
            write!(f, "{:.2}", self.0)
        }
    }
}


/// Component for the visuals of experience points.
#[derive(Component, Debug, Reflect)]
pub struct ExperiencePointVisuals {
    pub size: f32,
    pub color: Color,
}

impl Default for ExperiencePointVisuals {
    fn default() -> ExperiencePointVisuals {
        ExperiencePointVisuals {
            size: DEFAULT_EXPERIENCE_POINT_SIZE,
            color: DEFAULT_EXPERIENCE_POINT_COLOR,
        }
    }
}


/// Component for the attraction speed of experience points.
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct ExperiencePointAttractionSpeed(pub AttractionSpeed);

impl Default for ExperiencePointAttractionSpeed {
    fn default() -> ExperiencePointAttractionSpeed {
        ExperiencePointAttractionSpeed(AttractionSpeed::Accelerating {
            min_speed: DEFAULT_EXPERIENCE_POINT_MIN_SPEED,
            acceleration_per_second: DEFAULT_EXPERIENCE_POINT_ACCELERATION_PER_SECOND,
            current_speed: DEFAULT_EXPERIENCE_POINT_MIN_SPEED,
            max_speed: DEFAULT_EXPERIENCE_POINT_MAX_SPEED,
        })
    }
}


/// Tag component for experience points.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ExperiencePoint;


/// Bundle for experience points.
#[derive(Bundle)]
pub struct ExperiencePointBundle {
    pub position: Position,
    pub attraction_speed: AttractionSpeed,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub collider: Collider,
    pub experience: Experience,
}

impl ExperiencePointBundle {
    /// Spawns an experience point.
    pub fn spawn<'c>(
        self,
        commands: &'c mut Commands,
        counter: &mut ExperiencePointCounter,
    ) -> EntityCommands<'c> {
        counter.increment();
        commands.spawn((
            // Tags
            Name::new(format!("Experience Point {}", counter.get())),
            ExperiencePoint,
            // Experience Point
            self,
            // Physics
            RigidBody::Kinematic,
            Restitution::PERFECTLY_INELASTIC,
            LinearVelocity::ZERO,
            LockedAxes::ROTATION_LOCKED,
            CollisionLayers::new(
                [Layer::ExperiencePoint],
                [Layer::Player, Layer::PlayerPickupArea],
            ),
        ))
    }
}
