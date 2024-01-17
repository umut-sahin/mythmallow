use crate::prelude::*;


/// Interface for items.
pub trait IItem: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the item.
    fn id(&self) -> SmolStr;
    /// Gets the name of the item.
    fn name(&self) -> SmolStr;

    /// Gets if the item is a weapon.
    fn is_weapon(&self) -> bool;

    /// Instantiates the item to add it to the inventory.
    fn instantiate(&self) -> ItemInstance;
    // Acquires the item.
    fn acquire(&self, world: &mut World) -> Entity;
    // Releases the item.
    fn release(&self, world: &mut World, entity: Entity);
}
