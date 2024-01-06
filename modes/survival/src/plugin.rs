use {
    crate::{
        prelude::*,
        systems::*,
    },
    mythmallow::prelude::*,
};

/// Plugin for managing "Survival" game mode.
pub struct SurvivalModePlugin;

impl Plugin for SurvivalModePlugin {
    fn build(&self, app: &mut App) {
        // Register the game mode.
        let mut game_mode_registry = app.world.resource_mut::<GameModeRegistry>();
        game_mode_registry.register(Survival);

        // Register resources.
        app.register_type::<CurrentWave>();
        app.register_type::<GameMode<Survival>>();
        app.register_type::<Survival>();
        app.register_type::<WaveTimer>();

        // Add initialization systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            initialize.in_set(InitializationSystems::GameMode).run_if(in_game_mode::<Survival>),
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
