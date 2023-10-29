use crate::prelude::*;


/// Tag component for basic attacks.
#[derive(Component, Debug, Reflect)]
pub struct Attack;


/// Component for remaining health.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct RemainingHealth(pub f32);
