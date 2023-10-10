use crate::{
    movement::systems::*,
    prelude::*,
};

/// Plugin for managing movement of game objects.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Dashing>();
        app.register_type::<Cooldown<Dashing>>();

        app.add_systems(PreUpdate, keep_dashing.in_set(GamePlaySystems::Movement));
        app.add_systems(PostUpdate, start_dashing.in_set(GamePlaySystems::Movement));
    }
}
