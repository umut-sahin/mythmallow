use crate::prelude::*;


/// Database of registered leveling systems.
#[derive(Clone, Copy, Debug, Deref, DerefMut, Resource)]
pub struct RegisteredLevelingSystems {
    pub set_level: SystemId<Level>,
}

impl RegisteredLevelingSystems {
    /// Creates the database.
    pub fn new(app: &mut App, systems: Entity) -> RegisteredLevelingSystems {
        use super::systems::*;

        let set_level = app.world.register_system(set_level);
        RegisteredSystems::attach(app, systems, set_level, "set_level");

        RegisteredLevelingSystems { set_level }
    }
}


/// Resource for counting spawned experience points.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct ExperiencePointCounter(usize);

impl ExperiencePointCounter {
    /// Gets the dropped experience point count.
    pub fn get(&self) -> usize {
        self.0
    }

    /// Increments the dropped experience point count.
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}


/// Resource for the player level structure for the selected game mode.
#[derive(Clone, Resource)]
pub struct PlayerLevelStructure {
    /// Max level that can be reached in the game mode.
    pub max_level: Option<Level>,
    /// Function to calculate the required experience for each level.
    pub required_experience_calculator: fn(&World, Level) -> ExperienceRequiredToLevelUp,
}


/// Resource for the experience required to get to current level.
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct ExperienceRequiredToGetToCurrentLevel(pub Experience);


/// Resource for the experience required to level up.
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct ExperienceRequiredToLevelUp(pub Experience);
