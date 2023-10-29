use crate::prelude::*;


/// Interface for mythologies.
pub trait Mythology: Any + Debug + Send + Sync + 'static {
    fn id(&self) -> SmolStr;
    fn name(&self) -> SmolStr;
}


/// Interface for players.
pub trait Playable: Debug + Send + Sync + 'static {
    fn id(&self) -> SmolStr;
    fn name(&self) -> SmolStr;
    fn spawn(&self, world: &mut World);
}
