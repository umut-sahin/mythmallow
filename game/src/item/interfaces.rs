use crate::{
    item::constants::*,
    prelude::*,
};


/// Interface for items.
pub trait IItem: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the item.
    fn id(&self) -> SmolStr;
    /// Gets the localized name of the item.
    fn name(&self) -> LocalizedText;

    /// Gets if the item is a weapon.
    fn is_weapon(&self) -> bool;
    /// Gets the base range of the weapon.
    fn base_range(&self) -> Option<Range> {
        if self.is_weapon() {
            panic!("weapons need to provide a base range");
        } else {
            None
        }
    }

    /// Gets whether the item needs to be whitelisted explicitly to appear in the market.
    fn needs_to_be_whitelisted_to_appear_in_market(&self) -> bool {
        false
    }
    /// Gets the commonness of the item in the market.
    fn commonness(&self) -> u64 {
        DEFAULT_ITEM_COMMONNESS
    }
    /// Gets the base price of the item in the market.
    fn base_price(&self) -> Balance;

    /// Instantiates the item to add it to the inventory.
    fn instantiate(&self) -> ItemInstance;
    // Acquires the item.
    fn acquire(&self, world: &mut World) -> Entity;
    // Releases the item.
    fn release(&self, world: &mut World, entity: Entity);
}
