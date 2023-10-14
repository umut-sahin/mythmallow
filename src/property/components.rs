use crate::prelude::*;


/// Property component for damage.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Damage(pub f32);


/// Property component for maximum health.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Health(pub f32);


/// Property component for speed.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Speed(pub f32);
