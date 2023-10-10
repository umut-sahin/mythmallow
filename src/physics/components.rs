use crate::prelude::*;


/// Physics component for position of physics entities.
#[derive(Clone, Component, Copy, Debug, Default, Deref, DerefMut, Reflect)]
pub struct Position(pub Vec2);


/// Physics component for velocities of physics entities.
#[derive(Clone, Component, Copy, Debug, Default, Deref, DerefMut, Reflect)]
pub struct Velocity(pub Vec2);


/// Physics component for colliders of physics entities.
#[derive(Clone, Component, Copy, Debug, Reflect)]
pub struct Collider {
    /// Radius of the collider.
    pub radius: f32,
}

impl Default for Collider {
    fn default() -> Collider {
        Collider { radius: 0.50 }
    }
}


/// Physics component to disable collision resolution for physics entities.
#[derive(Clone, Component, Copy, Debug, Reflect)]
pub struct Floating;


/// Bundle for physics entities.
#[derive(Bundle, Debug)]
pub struct PhysicsBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub collider: Collider,
}

impl PhysicsBundle {
    /// Creates a new physics entities with the given position.
    pub fn at(x: f32, y: f32) -> PhysicsBundle {
        PhysicsBundle {
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::ZERO),
            collider: Collider::default(),
        }
    }

    /// Sets the velocity of the physics entities.
    pub fn with_velocity(mut self, x: f32, y: f32) -> PhysicsBundle {
        self.velocity = Velocity(Vec2::new(x, y));
        self
    }

    /// Sets the collider of the physics entities.
    pub fn with_collider(mut self, collider: Collider) -> PhysicsBundle {
        self.collider = collider;
        self
    }
}
