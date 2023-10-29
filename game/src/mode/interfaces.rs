use crate::prelude::*;


/// Interface for game modes.
pub trait Mode: Debug + Send + Sync + 'static {
    fn id(&self) -> SmolStr;
    fn name(&self) -> SmolStr;
    fn initialize(&self, world: &mut World);
    fn deinitialize(&self, world: &mut World);
}
