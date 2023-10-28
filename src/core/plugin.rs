use crate::{
    core::systems::*,
    prelude::*,
};

/// Plugin for the core logic of the application.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        let args = app.world.resource::<Args>();

        let seed = {
            let seed = match args.seed {
                Some(seed) => seed,
                None => ChaCha8Rng::from_entropy().gen::<u64>(),
            };
            log::info!("seeding {}", seed);
            ChaCha8Rng::seed_from_u64(seed).gen::<[u8; 32]>()
        };

        app.add_plugins(EntropyPlugin::<ChaCha8Rng>::with_seed(seed));

        app.register_type::<AppState>();
        app.register_type::<GameState>();
        app.register_type::<GameResult>();
        app.register_type::<DiagnosticsOverlayState>();

        app.add_state::<AppState>();
        app.add_state::<GameState>();
        app.add_state::<DiagnosticsOverlayState>();

        let general_settings = app.world.resource::<Persistent<GeneralSettings>>();
        if general_settings.show_diagnostics_overlay {
            app.world.insert_resource(NextState(Some(DiagnosticsOverlayState::Enabled)));
        }

        MainMenuSystems::configure(app);
        GameModeSelectionScreenSystems::configure(app);
        InitializationSystems::configure(app);
        LoadingSystems::configure(app);
        GameplaySystems::configure(app);
        PauseMenuSystems::configure(app);
        GameOverMenuSystems::configure(app);
        RestartSystems::configure(app);
        MenuSystems::configure(app);

        app.add_systems(
            OnEnter(GameState::Initialization),
            start_loading.in_set(InitializationSystems::Done),
        );
        app.add_systems(OnEnter(GameState::Loading), start_playing.in_set(LoadingSystems::Done));
        app.add_systems(OnEnter(GameState::Restart), restart.in_set(RestartSystems::Done));
    }
}
