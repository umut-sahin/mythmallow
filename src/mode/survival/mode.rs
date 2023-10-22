use crate::prelude::*;


/// Survival game mode.
#[derive(Debug, Default, Reflect, Resource)]
pub struct Survival;

impl Mode for Survival {
    fn name(&self) -> &'static str {
        "Survival"
    }

    fn description(&self) -> &'static str {
        todo!();
    }

    fn initialize(&self, world: &mut World) {
        world.init_resource::<GameMode<Survival>>();
    }

    fn deinitialize(&self, world: &mut World) {
        world.remove_resource::<GameMode<Survival>>();
    }
}
