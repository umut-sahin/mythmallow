use {
    crate::constants::*,
    mythmallow::prelude::*,
};

/// Tag component for the item "Bident of Hades".
#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct BidentOfHades;

impl IItem for BidentOfHades {
    fn id(&self) -> SmolStr {
        "bident-of-hades".into()
    }

    fn name(&self) -> SmolStr {
        "Bident of Hades".into()
    }

    fn is_weapon(&self) -> bool {
        true
    }

    fn instantiate(&self) -> ItemInstance {
        ItemInstance::new(self.clone())
    }

    fn acquire(&self, world: &mut World) -> Entity {
        world.run_system_once_with(self.clone(), acquire)
    }

    fn release(&self, world: &mut World, entity: Entity) {
        world.run_system_once_with(entity, release);
    }
}

/// Plugin for managing the item "Bident of Hades".
pub struct BidentOfHadesPlugin;

impl Plugin for BidentOfHadesPlugin {
    fn build(&self, app: &mut App) {
        // Register the item.
        let mut item_registry = app.world.resource_mut::<ItemRegistry>();
        item_registry.register(BidentOfHades).add_tag(GREEK_ITEM_TAG);

        // Register components.
        app.register_type::<BidentOfHades>();
    }
}

/// Acquires the item.
pub fn acquire(
    In(item): In<BidentOfHades>,
    mut commands: Commands,
    inventory: Res<Inventory>,
) -> Entity {
    commands
        .spawn((
            Name::new(format!("Item {} [{}]", inventory.items.len(), item.name().to_string())),
            item,
        ))
        .id()
}

/// Releases the item.
pub fn release(In(entity): In<Entity>, mut commands: Commands) {
    if let Some(entity) = commands.get_entity(entity) {
        entity.despawn_recursive();
    }
}
