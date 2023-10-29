use crate::prelude::*;


/// Interface for items.
pub trait Item: Debug + Send + Sync + 'static {
    fn id(&self) -> SmolStr;
    fn name(&self) -> SmolStr;
    fn instantiate(&self) -> ItemInstance;
    fn acquire(&self, world: &mut World) -> Option<Entity>;
    fn release(&self, world: &mut World, entity: Option<Entity>);
}
