use crate::prelude::*;


/// Container for the items in the inventory.
#[derive(Debug)]
pub struct ItemInstance {
    pub item: Box<dyn IItem>,
    pub entity: Option<Entity>,
}

impl ItemInstance {
    /// Creates a new item instance.
    pub fn new(item: impl IItem) -> ItemInstance {
        ItemInstance { item: Box::new(item), entity: None }
    }
}

impl Deref for ItemInstance {
    type Target = Box<dyn IItem>;

    fn deref(&self) -> &Box<dyn IItem> {
        &self.item
    }
}


/// Resource for the inventory.
#[derive(Debug, Default, Resource)]
pub struct Inventory {
    pub items: Vec<Arc<ItemInstance>>,
    pub items_to_add: Vec<ItemInstance>,
    pub items_to_remove: Vec<Arc<ItemInstance>>,
}

impl Inventory {
    /// Adds an item to the inventory.
    pub fn add(&mut self, item: ItemInstance) {
        self.items_to_add.push(item);
    }

    /// Removes an item from the inventory.
    pub fn remove(&mut self, item: Arc<ItemInstance>) {
        self.items_to_remove.push(item);
    }
}

impl Deref for Inventory {
    type Target = Vec<Arc<ItemInstance>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
