use mythmallow::prelude::*;

/// Survival game mode.
#[derive(Debug, Default, Reflect, Resource)]
pub struct Survival;

impl Mode for Survival {
    fn id(&self) -> SmolStr {
        "survival".into()
    }

    fn name(&self) -> SmolStr {
        "Survival".into()
    }

    fn initialize(&self, world: &mut World) {
        world.init_resource::<GameMode<Survival>>();
    }

    fn deinitialize(&self, world: &mut World) {
        world.remove_resource::<GameMode<Survival>>();
    }
}
