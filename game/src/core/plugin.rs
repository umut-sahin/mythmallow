use crate::{
    core::systems::*,
    prelude::*,
};

/// Plugin for managing the core infrastructure of the application.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Rarity>();

        // Setup easing.
        app.add_plugins(EasingsPlugin);

        // Setup the global random number generator.
        let seed = {
            let args = app.world.resource::<Args>();
            let seed = match args.seed {
                Some(seed) => seed,
                None => ChaCha8Rng::from_entropy().gen::<u64>(),
            };

            log::info!("seeding {}", seed);
            ChaCha8Rng::seed_from_u64(seed).gen::<[u8; 32]>()
        };
        app.add_plugins(EntropyPlugin::<ChaCha8Rng>::with_seed(seed));

        // Register states.
        app.register_type::<AppState>();
        app.register_type::<GameState>();
        app.register_type::<LocalizationState>();
        app.register_type::<DiagnosticsOverlayState>();

        // Add states to the application.
        app.init_state::<AppState>();
        app.init_state::<GameState>();
        app.init_state::<LocalizationState>();
        app.init_state::<DiagnosticsOverlayState>();

        // Enable diagnostics overlay if it's enabled in the general settings.
        let general_settings = app.world.resource::<Persistent<GeneralSettings>>();
        if general_settings.show_diagnostics_overlay {
            app.world.insert_resource(NextState(Some(DiagnosticsOverlayState::Enabled)));
        }

        // Configure system sets.
        MenuSystems::configure(app);
        MainMenuSystems::configure(app);
        SettingsMenuSystems::configure(app);
        GameModeSelectionScreenSystems::configure(app);
        PlayerSelectionScreenSystems::configure(app);
        EnemySelectionScreenSystems::configure(app);
        InitializationSystems::configure(app);
        LoadingSystems::configure(app);
        GameplaySystems::configure(app);
        LevelUpScreenSystems::configure(app);
        MarketSystems::configure(app);
        PauseMenuSystems::configure(app);
        RestartSystems::configure(app);
        GameOverMenuSystems::configure(app);

        // Register resources.
        app.register_type::<GameStateStack>();
        app.register_type::<GameResult>();

        // Insert resources.
        app.init_resource::<GameStateStack>();

        // Register systems.
        let registered_systems = RegisteredSystems::new(app);
        app.insert_resource(registered_systems);

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            start_loading.in_set(InitializationSystems::Done),
        );
        app.add_systems(OnEnter(GameState::Loading), start_playing.in_set(LoadingSystems::Done));
        app.add_systems(OnEnter(GameState::Restart), restart.in_set(RestartSystems::Done));
        app.add_systems(
            Update,
            game_state_transition
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Transition)),
        );
        app.add_systems(OnExit(AppState::Game), reset_game_state_stack);
    }
}
