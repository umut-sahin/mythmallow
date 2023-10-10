use crate::prelude::*;

/// Plugin for the properties of game objects.
pub struct PropertyPlugin;

impl Plugin for PropertyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>();
    }
}
