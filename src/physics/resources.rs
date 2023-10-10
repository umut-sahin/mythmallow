use crate::prelude::*;


/// Physics container that contains collision information.
#[derive(Clone, Copy, Debug, Reflect)]
pub struct Collision {
    /// Colliding entities.
    pub entities: (Entity, Entity),
    /// Whether the colliding entities are actually overlapping
    /// or the collision is registered due to safety margin.
    pub is_overlapping: bool,
}

impl Collision {
    /// Creates a new collision between two entities.
    pub fn new(entity_a: Entity, entity_b: Entity) -> Collision {
        Collision { entities: (entity_a, entity_b), is_overlapping: true }
    }

    /// Sets the overlapping flag of the collision.
    pub fn overlapping(mut self, is_overlapping: bool) -> Collision {
        self.is_overlapping = is_overlapping;
        self
    }
}


/// Physics resource to track collisions in the latest physics update.
#[derive(Debug, Default, Deref, DerefMut, Reflect, Resource)]
pub struct Collisions(pub Vec<Collision>);
