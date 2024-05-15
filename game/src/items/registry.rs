use crate::prelude::*;


/// Container for the item registry.
#[derive(Debug, Default, Deref, Resource)]
pub struct ItemRegistry(Vec<ItemRegistryEntry>);

impl ItemRegistry {
    /// Registers an item to the item registry.
    pub fn register(&mut self, item: impl IItem) -> &mut RegisteredItem {
        let item_id = item.id();
        let item_index = match self.iter().position(|entry| entry.item.id() == item_id) {
            Some(index) => {
                log::warn!("tried to register {:?} to the item registry again", item_id);
                index
            },
            None => {
                log::info!("registered {:?} to the item registry", item_id);
                let index = self.len();
                self.0.push(ItemRegistryEntry::new(item));
                index
            },
        };
        &mut self.0[item_index].item
    }
}

impl ItemRegistry {
    /// Gets the number of items in the item registry.
    pub fn number_of_items(&self) -> usize {
        self.0.len()
    }

    /// Finds the item with the specified id.
    pub fn find_item_by_id(&self, item_id: &str) -> Option<&RegisteredItem> {
        for entry in self.iter() {
            if entry.item.id() == item_id {
                return Some(&entry.item);
            }
        }
        None
    }
}


/// Container for the entries of the iem registry.
#[derive(Debug)]
pub struct ItemRegistryEntry {
    pub item: RegisteredItem,
}

impl ItemRegistryEntry {
    /// Creates a new item registry entry.
    pub fn new(item: impl IItem) -> ItemRegistryEntry {
        ItemRegistryEntry { item: RegisteredItem::new(item) }
    }
}


/// Container for registered items.
#[derive(Clone, Debug)]
pub struct RegisteredItem {
    pub item: Arc<dyn IItem>,
    pub tags: SmallVec<[SmolStr; 3]>,
}

impl RegisteredItem {
    /// Creates a new registered item.
    pub fn new(item: impl IItem) -> RegisteredItem {
        RegisteredItem { item: Arc::new(item), tags: SmallVec::new() }
    }
}

impl RegisteredItem {
    /// Adds a tag to the item.
    pub fn add_tag(&mut self, tag: impl ToString) -> &mut RegisteredItem {
        self.tags.push(tag.to_string().into());
        self
    }
}

impl Deref for RegisteredItem {
    type Target = Arc<dyn IItem>;

    fn deref(&self) -> &Arc<dyn IItem> {
        &self.item
    }
}
