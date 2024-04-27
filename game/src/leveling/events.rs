use crate::prelude::*;


/// Event for gaining experience.
#[derive(Debug, Event, Reflect)]
pub struct ExperienceGainedEvent {
    pub entity: Entity,
    pub experience: Experience,
    pub by: String,
}


/// Event for levelling up.
#[derive(Debug, Event, Reflect)]
pub struct LeveledUpEvent {
    pub entity: Entity,
    pub new_level: Level,
}
