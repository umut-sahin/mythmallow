use crate::prelude::*;


/// Component for damage.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Damage(pub f32);


/// Component for maximum health.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Health(pub f32);


/// Component for experience point pickup range.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct PickupRange(pub f32);


/// Component for speed.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Speed(pub f32);
