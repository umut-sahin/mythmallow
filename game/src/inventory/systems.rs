use crate::prelude::*;


/// Acquires and releases items.
pub fn acquire_release_items(world: &mut World) {
    let mut inventory = world.resource_mut::<Inventory>();

    let items_to_acquire = std::mem::take(&mut inventory.items_to_add);
    let items_to_release = std::mem::take(&mut inventory.items_to_remove);

    for item_to_release in &items_to_release {
        if let Some(index) =
            inventory.items.iter().position(|item| Arc::ptr_eq(item, item_to_release))
        {
            inventory.items.remove(index);
        }
    }
    for item_to_release in items_to_release {
        item_to_release.release(world, item_to_release.entity);
    }

    let mut new_items = Vec::with_capacity(items_to_acquire.len());
    for mut item_to_acquire in items_to_acquire {
        item_to_acquire.entity = item_to_acquire.acquire(world);
        new_items.push(Arc::new(item_to_acquire));
    }

    let mut inventory = world.resource_mut::<Inventory>();
    inventory.items.extend(new_items);
}


/// Clears the inventory.
pub fn clear_inventory(world: &mut World) {
    let mut inventory = world.resource_mut::<Inventory>();

    inventory.items_to_add = Vec::new();
    inventory.items_to_remove = Vec::new();

    for item in std::mem::take(&mut inventory.items) {
        item.release(world, item.entity);
    }
}
