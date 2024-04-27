use crate::prelude::*;

/// Plugin for managing the properties of game objects.
pub struct PropertyPlugin;

impl Plugin for PropertyPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Damage>();
        app.register_type::<Health>();
        app.register_type::<PickupRange>();
        app.register_type::<Speed>();
    }
}
