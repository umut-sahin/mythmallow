use crate::{
    physics::{
        constants::*,
        schedules::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for physics of the game.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Position>();
        app.register_type::<Velocity>();
        app.register_type::<Collider>();
        app.register_type::<Floating>();
        app.register_type::<Collisions>();

        app.init_resource::<Collisions>();
        app.insert_resource(FixedTime::new_from_secs(DELTA_TIME));

        app.add_systems(
            FixedUpdate,
            (collect_collisions, run_substeps, confine_entities, update_transforms)
                .chain()
                .in_set(GamePlaySystems::Physics),
        );

        app.add_schedule(SubstepSchedule, Schedule::default());
        app.add_systems(
            SubstepSchedule,
            (apply_velocity, resolve_collisions).chain().in_set(GamePlaySystems::Physics),
        );
    }
}
