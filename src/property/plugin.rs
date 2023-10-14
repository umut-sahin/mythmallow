use crate::prelude::*;

/// Plugin for the properties of game objects.
pub struct PropertyPlugin;

impl Plugin for PropertyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Damage>();
        app.register_type::<Health>();
        app.register_type::<Speed>();
    }
}
