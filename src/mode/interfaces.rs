use crate::prelude::*;


/// Interface for game modes.
pub trait Mode: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn setup(&self, world: &mut World);
    fn cleanup(&self, world: &mut World);
}
