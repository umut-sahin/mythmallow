use crate::{
    prelude::*,
    systems::*,
};

/// Plugin for managing "Survival" game mode.
pub struct SurvivalModePlugin;

impl Plugin for SurvivalModePlugin {
    fn build(&self, app: &mut App) {
        // Register the game mode.
        let mut game_mode_registry = app.world.resource_mut::<GameModeRegistry>();
        game_mode_registry.register(Survival);

        // Register components.
        app.register_type::<CurrentWaveContainer>();
        app.register_type::<CurrentWaveText>();
        app.register_type::<RemainingSecondsContainer>();
        app.register_type::<RemainingSecondsText>();

        // Register resources.
        app.register_type::<CurrentWave>();
        app.register_type::<GameMode<Survival>>();
        app.register_type::<Survival>();
        app.register_type::<SurvivalModeArgs>();
        app.register_type::<WaveDurations>();
        app.register_type::<WaveTimer>();

        // Insert resources.
        let args = app.world.resource::<Args>();
        app.insert_resource(
            args.start_in_game_mode
                .as_ref()
                .filter(|game_mode_id_and_args| {
                    game_mode_id_and_args.starts_with(Survival.id().as_str())
                })
                .and_then(|game_mode_id_and_args| {
                    let mut split = game_mode_id_and_args.split(' ');

                    let game_mode = split.next().unwrap();
                    let args = split;

                    if game_mode == Survival.id() {
                        SurvivalModeArgs::parse(std::iter::once(game_mode).chain(args)).ok()
                    } else {
                        Some(SurvivalModeArgs::default())
                    }
                })
                .unwrap_or_default(),
        );

        // Add initialization systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            (
                initialize,
                (apply_deferred, select_wave_when_starting_in_game).chain().run_if(run_once()),
            )
                .chain()
                .in_set(InitializationSystems::GameMode)
                .run_if(in_game_mode::<Survival>),
        );

        // Add loading systems.
        app.add_systems(
            OnEnter(GameState::Loading),
            load.in_set(LoadingSystems::GameMode).run_if(in_game_mode::<Survival>),
        );
        app.add_systems(
            OnEnter(GameState::Loading),
            spawn_map.in_set(LoadingSystems::Map).run_if(in_game_mode::<Survival>),
        );

        // Add gameplay systems.
        app.add_systems(
            PreUpdate,
            tick.in_set(GameplaySystems::GameMode).run_if(in_game_mode::<Survival>),
        );
        app.add_systems(PostUpdate, (obtain_perk, level_change).run_if(in_game_mode::<Survival>));

        // Add game won systems.
        app.add_systems(OnEnter(GameState::Won), (unload, win).run_if(in_game_mode::<Survival>));

        // Add game over systems.
        app.add_systems(
            OnEnter(GameState::Over),
            (unload, deinitialize).run_if(in_game_mode::<Survival>),
        );

        // Add restart systems.
        app.add_systems(
            OnEnter(GameState::Restart),
            (unload, deinitialize)
                .in_set(RestartSystems::GameMode)
                .run_if(in_game_mode::<Survival>),
        );

        // Add exit systems.
        app.add_systems(
            OnExit(AppState::Game),
            (unload, deinitialize).run_if(in_game_mode::<Survival>),
        );
    }
}
