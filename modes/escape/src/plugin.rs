use {
    crate::{
        prelude::*,
        systems::*,
    },
    mythmallow::prelude::*,
};

/// Plugin for managing the "Escape" game mode.
pub struct EscapeModePlugin;

impl Plugin for EscapeModePlugin {
    fn build(&self, app: &mut App) {
        // Register the game mode.
        let mut game_mode_registry = app.world_mut().resource_mut::<GameModeRegistry>();
        game_mode_registry.register(Escape);

        // Register resources.
        app.register_type::<CurrentChapter>();
        app.register_type::<GameMode<Escape>>();
        app.register_type::<Escape>();
        app.register_type::<ChapterTimer>();

        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("content/modes/escape.ftl");

        // Add initialization systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            initialize.in_set(InitializationSystems::GameMode).run_if(in_game_mode::<Escape>),
        );

        // Add loading systems.
        app.add_systems(
            OnEnter(GameState::Loading),
            load.in_set(LoadingSystems::GameMode).run_if(in_game_mode::<Escape>),
        );
        app.add_systems(
            OnEnter(GameState::Loading),
            spawn_map.in_set(LoadingSystems::Map).run_if(in_game_mode::<Escape>),
        );

        // Add gameplay systems.
        app.add_systems(
            Update,
            tick.in_set(GameplaySystems::GameMode).run_if(in_game_mode::<Escape>),
        );

        // Add game won systems.
        app.add_systems(OnEnter(GameState::Won), unload.run_if(in_game_mode::<Escape>));

        // Add game over systems.
        app.add_systems(
            OnEnter(GameState::Over),
            (unload, deinitialize).run_if(in_game_mode::<Escape>),
        );

        // Add restart systems.
        app.add_systems(
            OnEnter(GameState::Restart),
            (unload, deinitialize).in_set(RestartSystems::GameMode).run_if(in_game_mode::<Escape>),
        );

        // Add exit systems.
        app.add_systems(
            OnExit(AppState::Game),
            (unload, deinitialize).run_if(in_game_mode::<Escape>),
        );
    }
}
