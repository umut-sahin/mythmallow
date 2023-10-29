use crate::{
    physics::systems::*,
    prelude::*,
};

/// Plugin for managing the physics of game objects.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Register layers.
        app.register_type::<Layer>();

        // Setup physics.
        app.insert_resource(Gravity::ZERO);
        app.add_plugins(XpbdPlugin::new(PostUpdate));

        // Pause physics in startup.
        app.world.resource_mut::<Time<Physics>>().pause();

        // Add systems.
        app.add_systems(OnEnter(GameState::Playing), resume_physics);
        app.add_systems(OnExit(GameState::Playing), pause_physics);
    }
}
