use crate::prelude::*;

/// Plugin for managing items.
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        // Initialize registry.
        app.init_resource::<ItemRegistry>();
    }
}
