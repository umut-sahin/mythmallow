use crate::prelude::*;


/// Property component for speed.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Speed(pub f32);
