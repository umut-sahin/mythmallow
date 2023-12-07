use mythmallow::prelude::*;

/// Resource for "Survival" game mode.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct Escape;

impl IGameMode for Escape {
    fn id(&self) -> SmolStr {
        "escape".into()
    }

    fn name(&self) -> SmolStr {
        "Escape".into()
    }

    fn default_enemy_spawn_pattern(&self, _world: &World) -> EnemySpawnPattern {
        let spawns = Vec::new();
        EnemySpawnPattern::new(spawns)
    }

    fn initialize(&self, world: &mut World) {
        world.init_resource::<GameMode<Escape>>();
    }

    fn deinitialize(&self, world: &mut World) {
        world.remove_resource::<GameMode<Escape>>();
    }
}
