use crate::{
    player::{
        commands::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing the player.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Player>();
        app.register_type::<DamagePlayerOnContact>();
        app.register_type::<DamagePlayerOnContactStarted>();
        app.register_type::<SelectedMythologyIndex>();
        app.register_type::<SelectedPlayerIndex>();

        // Register Resources.
        app.register_type::<GodMode>();

        // Insert resources.
        let args = app.world().resource::<Args>();
        app.insert_resource(GodMode { is_enabled: args.enable_god_mode });

        // Initialize registry.
        app.init_resource::<PlayerRegistry>();

        // Add console commands.
        app.add_console_command::<PlayerCommand, _>(apply_player_command);

        // Add systems.
        {
            app.add_systems(
                OnEnter(GameState::Initialization),
                spawn_player.in_set(InitializationSystems::Player),
            );

            app.add_systems(
                OnEnter(GameState::Loading),
                reset_player_position.in_set(LoadingSystems::Player),
            );
            app.add_systems(
                OnExit(GameState::Loading),
                turn_player_visibility_on.in_set(LoadingSystems::Player),
            );

            app.add_systems(PreUpdate, cooldown::<Dashing>.in_set(GameplaySystems::Player));
            app.add_systems(Update, (movement, dash).in_set(GameplaySystems::Player));
            app.add_systems(PostUpdate, pause.in_set(GameplaySystems::Player));

            app.add_systems(OnEnter(GameState::Won), turn_player_visibility_off);

            app.add_systems(OnEnter(GameState::Over), despawn_player);
            app.add_systems(
                OnEnter(GameState::Restart),
                despawn_player.in_set(RestartSystems::Player),
            );
            app.add_systems(OnExit(AppState::Game), despawn_player);

            app.add_systems(OnExit(AppState::Game), clear_player_selection);
        }
    }
}
