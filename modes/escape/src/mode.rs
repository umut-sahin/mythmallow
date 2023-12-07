use mythmallow::prelude::*;

/// Resource for "Survival" game mode.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct Escape;

impl IGameMode for Escape {
    fn id(&self) -> SmolStr {
        "escape".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "escape-mode-name",
            args: smallvec![],
            fallback: "Escape Mode".into(),
        }
    }

    fn market_can_be_opened_by_player(&self) -> bool {
        true
    }

    fn default_enemy_spawn_pattern(&self, _world: &World) -> EnemySpawnPattern {
        let spawns = Vec::new();
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
        world.init_resource::<GameMode<Escape>>();
    }

    fn deinitialize(&self, world: &mut World) {
        world.remove_resource::<GameMode<Escape>>();
    }
}
