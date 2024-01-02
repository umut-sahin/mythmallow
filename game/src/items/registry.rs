use crate::prelude::*;

/// Registry for items.
pub static ITEM_REGISTRY: Mutex<ItemRegistry> = Mutex::new(ItemRegistry::new());

/// Container for the item registry.
#[derive(Default, Deref, DerefMut, Resource)]
pub struct ItemRegistry(pub Vec<ItemRegistryEntry>);

impl ItemRegistry {
    /// Creates a new item registry.
    pub const fn new() -> ItemRegistry {
        ItemRegistry(Vec::new())
    }
}

impl ItemRegistry {
    /// Registers an item to the item registry.
    pub fn register(&mut self, item: impl IItem) -> &mut ItemRegistryEntry {
        let id = item.id();
        if self.iter().any(|entry| entry.id() == id) {
            log::warn!("tried to register {:?} to item registry again", item.id());
        } else {
            log::info!("registered {:?} to item registry", item.name());
            self.push(ItemRegistryEntry::new(item));
        }
        self.iter_mut().find(|entry| entry.id() == id).unwrap()
    }
}

impl ItemRegistry {
    /// Gets the number of items in the item registry.
    pub fn number_of_items(&self) -> usize {
        self.0.len()
    }
}

/// Container for item registry entries.
#[derive(Debug)]
pub struct ItemRegistryEntry {
    pub item: Arc<dyn IItem>,
    pub tags: SmallVec<[SmolStr; 3]>,
}

impl ItemRegistryEntry {
    /// Create a new entry for an item.
    pub fn new<I: IItem>(item: I) -> ItemRegistryEntry {
        ItemRegistryEntry { item: Arc::new(item), tags: SmallVec::new() }
    }
}

impl ItemRegistryEntry {
    /// Add a tag to the item.
    pub fn add_tag(&mut self, tag: impl ToString) -> &mut ItemRegistryEntry {
        self.tags.push(tag.to_string().into());
        self
    }
}

impl Deref for ItemRegistryEntry {
    type Target = Arc<dyn IItem>;

    fn deref(&self) -> &Arc<dyn IItem> {
        &self.item
    }
}
