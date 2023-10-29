use {
    crate::constants::*,
    mythmallow::prelude::*,
};

/// Tag component for the item "Bident of Hades".
#[derive(Clone, Component, Debug, Reflect)]
pub struct BidentOfHades;

impl Item for BidentOfHades {
    fn id(&self) -> SmolStr {
        "bident-of-hades".into()
    }

    fn name(&self) -> SmolStr {
        "Bident of Hades".into()
    }

    fn instantiate(&self) -> ItemInstance {
        ItemInstance::new(self.clone())
    }

    fn acquire(&self, _world: &mut World) -> Option<Entity> {
        None
    }

    fn release(&self, _world: &mut World, entity: Option<Entity>) {
        assert_eq!(entity, None);
    }
}

/// Plugin for managing the item "Bident of Hades".
pub struct BidentOfHadesPlugin;

impl Plugin for BidentOfHadesPlugin {
    fn build(&self, app: &mut App) {
        // Register the item.
        let mut item_registry = ITEM_REGISTRY.lock().unwrap();
        item_registry.register(BidentOfHades).add_tag(GREEK_ITEM_TAG);
        drop(item_registry);

        // Register resources.
        app.register_type::<BidentOfHades>();
    }
}
