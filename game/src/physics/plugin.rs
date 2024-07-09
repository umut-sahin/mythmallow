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
        app.add_plugins(AvianPlugin::default());

        // Setup physics gizmos in development mode.
        #[cfg(feature = "development")]
        {
            let general_settings = app.world().resource::<Persistent<GeneralSettings>>();
            app.insert_gizmo_config(
                PhysicsGizmos::default(),
                GizmoConfig { enabled: general_settings.enable_physics_gizmos, ..default() },
            );

            app.add_plugins(PhysicsDebugPlugin::default());
        }

        // Pause physics in startup.
        app.world_mut().resource_mut::<Time<Physics>>().pause();

        // Add systems.
        app.add_systems(OnEnter(GameState::Playing), resume_physics);
        app.add_systems(OnExit(GameState::Playing), pause_physics);
    }
}
