use crate::{
    prelude::*,
    ui::hud::systems::*,
};

/// Plugin for managing the HUD.
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Hud>();
        app.register_type::<HudHealthBar>();
        app.register_type::<HudHealthBarText>();
        app.register_type::<HudExperienceBar>();
        app.register_type::<HudExperienceBarText>();
        app.register_type::<HudBalanceContainer>();
        app.register_type::<HudBalanceText>();

        // Add materials.
        app.add_plugins(UiMaterialPlugin::<HealthBarMaterial>::default());
        app.add_plugins(UiMaterialPlugin::<ExperienceBarMaterial>::default());

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            spawn_hud.in_set(InitializationSystems::Hud),
        );
        app.add_systems(OnEnter(GameState::Playing), show_hud);
        app.add_systems(PostUpdate, update_health_bar.run_if(in_state(AppState::Game)));
        app.add_systems(
            PostUpdate,
            update_experience_bar
                .run_if(in_state(AppState::Game))
                .run_if(resource_exists::<ExperienceRequiredToGetToCurrentLevel>)
                .run_if(resource_exists::<ExperienceRequiredToLevelUp>),
        );
        app.add_systems(PostUpdate, update_balance.run_if(in_state(AppState::Game)));
        app.add_systems(OnExit(GameState::Playing), hide_hud);
        app.add_systems(OnEnter(GameState::Over), despawn_hud);
        app.add_systems(OnEnter(GameState::Restart), despawn_hud.in_set(RestartSystems::Hud));
        app.add_systems(OnExit(AppState::Game), despawn_hud);
    }
}
