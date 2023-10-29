use crate::{
    movement::systems::*,
    prelude::*,
};

/// Plugin for managing the movement of game objects.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Dashing>();
        app.register_type::<Cooldown<Dashing>>();

        // Add systems.
        app.add_systems(PreUpdate, keep_dashing.in_set(GameplaySystems::Movement));
        app.add_systems(PostUpdate, start_dashing.in_set(GameplaySystems::Movement));
    }
}
