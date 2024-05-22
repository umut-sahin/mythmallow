use {
    crate::prelude::*,
    mythmallow::enemy::constants::{
        MELEE_ENEMY_TAG,
        RANGED_ENEMY_TAG,
    },
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

    fn market_can_be_opened_by_player(&self) -> bool {
        false
    }

    fn default_enemy_spawn_pattern(&self, world: &World) -> EnemySpawnPattern {
        let enemy_registry = world.resource::<EnemyRegistry>();

        let selected_enemy_pack_index = world.resource::<SelectedEnemyPackIndex>();
        let enemies_in_selected_pack = &enemy_registry[*selected_enemy_pack_index].enemies;

        let first_melee_enemy = enemies_in_selected_pack
            .iter()
            .find(|enemy| enemy.has_tag(MELEE_ENEMY_TAG))
            .map(|enemy| enemy.deref());
        let first_ranged_enemy = enemies_in_selected_pack
            .iter()
            .find(|enemy| enemy.has_tag(RANGED_ENEMY_TAG))
            .map(|enemy| enemy.deref());

        let current_wave = world.resource::<CurrentWave>();

        let mut spawns = Vec::new();
        match current_wave.get() {
            1 => {
                if let Some(enemy) = first_melee_enemy {
                    spawns.push(
                        EnemySpawn::new_dyn(Duration::from_millis(500), enemy)
                            .count(3)
                            .interval(Duration::from_millis(150))
                            .spread(EnemySpawnSpread::square(100.00))
                            .repeat(Duration::from_millis(1500)),
                    );
                }
            },
            2 => {
                if let Some(enemy) = first_ranged_enemy {
                    spawns.push(
                        EnemySpawn::new_dyn(Duration::from_millis(500), enemy)
                            .count(3)
                            .interval(Duration::from_millis(150))
                            .spread(EnemySpawnSpread::square(100.00))
                            .repeat(Duration::from_millis(1500)),
                    );
                }
            },
            _ => {
                if let Some(enemy) = first_melee_enemy {
                    spawns.push(
                        EnemySpawn::new_dyn(Duration::from_millis(500), enemy)
                            .count(3)
                            .interval(Duration::from_millis(150))
                            .spread(EnemySpawnSpread::square(100.00))
                            .repeat(Duration::from_millis(3000)),
                    );
                }
                if let Some(enemy) = first_ranged_enemy {
                    spawns.push(
                        EnemySpawn::new_dyn(Duration::from_millis(1000), enemy)
                            .count(3)
                            .interval(Duration::from_millis(150))
                            .spread(EnemySpawnSpread::square(100.00))
                            .repeat(Duration::from_millis(3000)),
                    );
                }
            },
        }
        EnemySpawnPattern::new(spawns)
    }

    fn player_level_structure(&self) -> PlayerLevelStructure {
        PlayerLevelStructure {
            max_level: None,
            required_experience_calculator: |_world, level| {
                ExperienceRequiredToLevelUp(Experience(20.00 * level.get() as f64))
            },
        }
    }

    fn initialize(&self, world: &mut World) {
        world.init_resource::<GameMode<Survival>>();
    }

    fn deinitialize(&self, world: &mut World) {
        world.remove_resource::<GameMode<Survival>>();
    }
}
