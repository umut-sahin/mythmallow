use crate::prelude::*;


/// Component for the base orientation of weapons in the inventory.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct BaseOrientation(pub Quat);
