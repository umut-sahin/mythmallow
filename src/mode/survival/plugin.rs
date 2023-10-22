use {
    super::{
        mode::*,
        resources::*,
        systems::*,
    },
    crate::prelude::*,
};


/// Plugin for survival game mode.
pub struct SurvivalGameModePlugin;

impl Plugin for SurvivalGameModePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Survival>();
        app.register_type::<GameMode<Survival>>();

        let mut game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
        game_mode_registry.register(Survival);
        drop(game_mode_registry);

        app.register_type::<CurrentWave>();
        app.register_type::<WaveTimer>();

        let in_survival_mode =
            |survival_mode: Option<Res<GameMode<Survival>>>| survival_mode.is_some();

        app.add_systems(
            OnEnter(GameState::Initialization),
            initialize.in_set(InitializationSystems::GameMode).run_if(in_survival_mode),
        );
        app.add_systems(
            OnEnter(GameState::Loading),
            load.in_set(LoadingSystems::GameMode).run_if(in_survival_mode),
        );
        app.add_systems(
            OnEnter(GameState::Loading),
            spawn_map.in_set(LoadingSystems::Map).run_if(in_survival_mode),
        );
        app.add_systems(Update, tick.in_set(GameplaySystems::GameMode).run_if(in_survival_mode));
        app.add_systems(OnEnter(GameState::Won), win.run_if(in_survival_mode));
    }
}
