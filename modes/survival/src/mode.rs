use mythmallow::{
    enemy::constants::MELEE_ENEMY_TAG,
    prelude::*,
};

/// Resource for "Survival" game mode.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct Survival;

impl IGameMode for Survival {
    fn id(&self) -> SmolStr {
        "survival".into()
    }

    fn name(&self) -> SmolStr {
        "Survival".into()
    }

    fn default_enemy_spawn_pattern(&self, world: &World) -> EnemySpawnPattern {
        let enemy_registry = world.resource::<EnemyRegistry>();

        let selected_enemy_pack_index = world.resource::<SelectedEnemyPackIndex>();
        let enemies_in_selected_pack = &enemy_registry[*selected_enemy_pack_index].enemies;

        let enemy = enemies_in_selected_pack
            .iter()
            .find(|enemy| enemy.has_tag(MELEE_ENEMY_TAG))
            .map(|enemy| enemy.deref());

        let mut spawns = Vec::new();
        if let Some(enemy) = enemy {
            spawns.push(
                EnemySpawn::new_dyn(Duration::from_millis(500), enemy)
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
