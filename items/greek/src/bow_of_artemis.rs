use {
    crate::constants::*,
    mythmallow::prelude::*,
};

/// Tag component for the item "Bow of Artemis".
#[derive(Clone, Component, Debug, Reflect)]
pub struct BowOfArtemis;

impl Item for BowOfArtemis {
    fn id(&self) -> SmolStr {
        "bow-of-artemis".into()
    }

    fn name(&self) -> SmolStr {
        "Bow of Artemis".into()
    }

    fn instantiate(&self) -> ItemInstance {
        ItemInstance::new(self.clone())
    }

    fn acquire(&self, _world: &mut World) -> Option<Entity> {
        None
    }

    fn release(&self, _world: &mut World, entity: Option<Entity>) {
        assert!(entity.is_none());
    }
}

/// Plugin for managing the item "Bow of Artemis".
pub struct BowOfArtemisPlugin;

impl Plugin for BowOfArtemisPlugin {
    fn build(&self, app: &mut App) {
        // Register the item.
        let mut item_registry = ITEM_REGISTRY.lock().unwrap();
        item_registry.register(BowOfArtemis).add_tag(GREEK_ITEM_TAG);
        drop(item_registry);

        // Register resources.
        app.register_type::<BowOfArtemis>();
    }
}
