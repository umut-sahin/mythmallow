use mythmallow::{
    enemy::constants::MELEE_ENEMY_TAG,
    prelude::*,
};

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

    fn default_enemy_spawn_pattern(&self, world: &World) -> EnemySpawnPattern {
        let (_, enemies) = world.resource::<SelectedEnemyPack>().deref();
        let enemy =
            enemies.iter().find(|enemy| enemy.has_tag(MELEE_ENEMY_TAG)).map(|enemy| enemy.deref());

        let mut spawns = Vec::new();
        if let Some(enemy) = enemy {
            spawns.push(
                EnemySpawn::new(Duration::from_millis(500), enemy)
                    .count(3)
                    .interval(Duration::from_millis(150))
                    .spread(EnemySpawnSpread::square(100.00))
                    .repeat(Duration::from_millis(1500)),
            );
        }
        EnemySpawnPattern::new(spawns)
    }

    fn initialize(&self, world: &mut World) {
        world.init_resource::<GameMode<Survival>>();
    }

    fn deinitialize(&self, world: &mut World) {
        world.remove_resource::<GameMode<Survival>>();
    }
}
