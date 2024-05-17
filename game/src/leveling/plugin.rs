use crate::{
    leveling::{
        commands::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing leveling.
pub struct LevelingPlugin;

impl Plugin for LevelingPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Level>();
        app.register_type::<Experience>();
        app.register_type::<ExperiencePointVisuals>();
        app.register_type::<ExperiencePointAttractionSpeed>();
        app.register_type::<ExperiencePoint>();

        // Register resources.
        app.register_type::<EnemyCounter>();
        app.register_type::<ExperienceRequiredToGetToCurrentLevel>();
        app.register_type::<ExperienceRequiredToLevelUp>();

        // Add events.
        app.add_event::<ExperienceGainedEvent>();
        app.add_event::<LeveledUpEvent>();

        // Add console commands.
        app.add_console_command::<ExperienceCommand, _>(apply_experience_command);
        app.add_console_command::<LevelCommand, _>(apply_level_command);

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            initialize_player_level_structure.in_set(InitializationSystems::Leveling),
        );
        app.add_systems(
            OnEnter(GameState::Loading),
            initialize_experience_point_counter.in_set(LoadingSystems::Leveling),
        );
        app.add_systems(
            Update,
            (attract_experience_points, collect_experience_points)
                .chain()
                .in_set(GameplaySystems::Leveling),
        );
        app.add_systems(
            PostUpdate,
            (
                gain_player_experience,
                level_player_up.run_if(
                    |player_query: Query<(), (With<Player>, Changed<Experience>)>| {
                        !player_query.is_empty()
                    },
                ),
            )
                .chain(),
        );
        app.add_systems(
            OnEnter(GameState::Won),
            (despawn_experience_points, clear_experience_point_counter),
        );
        app.add_systems(
            OnEnter(GameState::Over),
            (
                despawn_experience_points,
                clear_experience_point_counter,
                clear_player_level_structure,
            ),
        );
        app.add_systems(
            OnEnter(GameState::Restart),
            (
                despawn_experience_points,
                clear_experience_point_counter,
                clear_player_level_structure,
            )
                .in_set(RestartSystems::Leveling),
        );
        app.add_systems(
            OnExit(AppState::Game),
            (
                despawn_experience_points,
                clear_experience_point_counter,
                clear_player_level_structure,
            ),
        );
    }
}
