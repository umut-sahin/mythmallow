use crate::{
    item::{
        commands::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing items.
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Item>();
        app.register_type::<Weapon>();

        // Initialize registry.
        app.init_resource::<ItemRegistry>();

        // Add console commands.
        app.add_console_command::<ItemCommand, _>(apply_item_command);
    }
}
