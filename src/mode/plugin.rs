use crate::{
    mode::{
        survival::plugin::SurvivalGameModePlugin,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing game modes.
pub struct ModePlugin;

impl Plugin for ModePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameModeIndex>();

        app.add_plugins(SurvivalGameModePlugin);

        app.add_systems(
            PostUpdate,
            game_mode_selected
                .in_set(GameModeSelectionScreenSystems)
                .run_if(|game_mode_index: Option<Res<GameModeIndex>>| game_mode_index.is_some()),
        );

        app.add_systems(OnEnter(GameState::Setup), setup_game_mode.in_set(SetupSystems::First));
        app.add_systems(
            OnEnter(GameState::Restart),
            restart_game_mode.in_set(RestartSystems::Last),
        );
        app.add_systems(OnExit(AppState::Game), cleanup_game_mode);

        let args = app.world.resource::<Args>();
        if args.start_in_game {
            app.add_systems(
                OnEnter(AppState::GameModeSelectionScreen),
                select_game_mode_when_starting_in_game.run_if(run_once()),
            );
        }
    }
}
