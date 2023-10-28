use crate::prelude::*;


/// Physics layers to differentiate collisions.
#[derive(Debug, PhysicsLayer, Reflect)]
pub enum Layer {
    Player,
    Enemy,
}
