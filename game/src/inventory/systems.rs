use crate::{
    inventory::commands::*,
    prelude::*,
};


/// Applies the inventory console commands.
pub fn apply_inventory_command(
    mut inventory: ResMut<Inventory>,
    item_registry: Res<ItemRegistry>,
    mut command: ConsoleCommand<InventoryCommand>,
) {
    if let Some(Ok(InventoryCommand { subcommand })) = command.take() {
        match subcommand {
            InventoryCommands::List => {
                if inventory.is_empty() {
                    reply!(command, "Inventory is empty.");
                } else {
                    for (i, item) in inventory.iter().enumerate() {
                        reply!(command, "{}) {}", i + 1, item.id());
                    }
                }
            },
            InventoryCommands::Add { item } => {
                match item_registry.find_item_by_id(&item) {
                    Some(item) => {
                        inventory.add(item.instantiate());
                        reply!(command, "Added.");
                    },
                    None => {
                        reply!(
                            command,
                            "Failed to add {:?} to the inventory as it doesn't exist.",
                            item,
                        );
                        reply!(command, "Run \"item list\" to see available items.")
                    },
                }
            },
        }
        reply!(command, "");
    }
}


/// Adds the items specified in the inventory argument to the inventory.
pub fn load_inventory_when_starting_in_game(
    args: Res<Args>,
    item_registry: Res<ItemRegistry>,
    mut inventory: ResMut<Inventory>,
) {
    if !args.start_in_game {
        return;
    }

    if !args.start_in_game_inventory.is_empty() {
        log::info!("initializing the inventory");
        for item_id in &args.start_in_game_inventory {
            match item_registry.find_item_by_id(item_id) {
                Some(item) => {
                    inventory.add(item.instantiate());
                },
                None => {
                    log::error!(
                        "unable to add \"{}\" to the inventory as it's registered",
                        item_id,
                    );
                },
            }
        }
    }
}


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
        if let Some(entity) = item_to_release.entity {
            item_to_release.release(world, entity);
        }
    }

    let mut new_items = Vec::with_capacity(items_to_acquire.len());
    for mut item_to_acquire in items_to_acquire {
        let new_item_entity = item_to_acquire.acquire(world);

        let mut new_item_entity_commands = world.entity_mut(new_item_entity);
        new_item_entity_commands.insert(Item);

        if item_to_acquire.is_weapon() {
            new_item_entity_commands.insert(Weapon);
        }

        item_to_acquire.entity = Some(new_item_entity);
        new_items.push(Arc::new(item_to_acquire));
    }

    if let Ok(player_entity) = world.query_filtered::<Entity, With<Player>>().get_single(world) {
        for new_item in &new_items {
            if let Some(new_item_entity) = new_item.entity {
                world.entity_mut(player_entity).add_child(new_item_entity);
            }
        }
    }

    let mut inventory = world.resource_mut::<Inventory>();
    inventory.items.extend(new_items);
}


/// Repositions the weapons around the player.
pub fn reposition_weapons(
    player_query: Query<(&GlobalTransform, &Collider), With<Player>>,
    mut weapon_query: Query<&mut Transform, (With<Weapon>, Without<Player>)>,
    mut spatial_query: SpatialQuery,
) {
    spatial_query.update_pipeline();

    if weapon_query.is_empty() {
        return;
    }

    log::info!("repositioning weapons");

    let (player_global_transform, player_collider) = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let player_position = player_global_transform.translation().xy();

    let player_aabb = player_collider.aabb(player_position.xy(), 0.00);
    let max_distance = player_aabb.min.distance(player_aabb.max);

    let mut direction = Vec2::X;
    let rotation = Rotation::from_degrees(360.00 / (weapon_query.iter().len() as f32));

    for mut weapon_transform in weapon_query.iter_mut() {
        let distance = spatial_query
            .cast_ray(
                player_position.xy(),
                Direction2d::new(direction).unwrap(),
                max_distance,
                false,
                SpatialQueryFilter::from_mask([Layer::Player]),
            )
            .map(|hit| hit.time_of_impact)
            .unwrap_or(max_distance);

        let new_weapon_translation = (direction * distance) * 1.05;

        weapon_transform.translation.x = new_weapon_translation.x;
        weapon_transform.translation.y = new_weapon_translation.y;

        direction = rotation.rotate(direction);
    }
}


/// Clears the inventory.
pub fn clear_inventory(world: &mut World) {
    let mut inventory = world.resource_mut::<Inventory>();

    inventory.items_to_add = Vec::new();
    inventory.items_to_remove = Vec::new();

    for item in std::mem::take(&mut inventory.items) {
        if let Some(entity) = item.entity {
            item.release(world, entity);
        }
    }
}
