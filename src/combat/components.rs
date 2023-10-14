use crate::prelude::*;


/// Combat component for basic attacks.
#[derive(Component, Debug, Reflect)]
pub struct Attack;


/// Combat component for remaining health.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct RemainingHealth(pub f32);
