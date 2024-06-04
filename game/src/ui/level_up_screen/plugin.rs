use crate::{
    prelude::*,
    ui::level_up_screen::{
        commands::*,
        systems::*,
    },
};

/// Plugin for managing the level up screen.
pub struct LevelUpScreenPlugin;

impl Plugin for LevelUpScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<LevelUpScreen>();
        app.register_type::<LevelUpScreenPerksContainer>();
        app.register_type::<LevelUpScreenPerkDetails>();
        app.register_type::<LevelUpScreenPerkNameText>();
        app.register_type::<LevelUpScreenPerkDescriptionText>();
        app.register_type::<LevelUpScreenFooterContainer>();
        app.register_type::<LevelUpScreenBalanceContainer>();
        app.register_type::<LevelUpScreenBalanceText>();
        app.register_type::<LevelUpScreenRerollButton>();

        // Register resources.
        app.register_type::<LevelUpScreenConfiguration>();
        app.register_type::<LevelUpScreenState>();
        app.register_type::<LevelUpScreenWidgets>();
        app.register_type::<PreviouslySelectedLevelUpScreenWidget>();

        // Insert resources.
        app.init_resource::<LevelUpScreenConfiguration>();

        // Add console commands.
        app.add_console_command::<LevelUpScreenCommand, _>(apply_level_up_screen_command);

        // Add systems.
        app.add_systems(OnEnter(GameState::LevelUpScreen), spawn_level_up_screen);
        app.add_systems(
            PreUpdate,
            (
                update_level_up_screen_widget_hierarchy.run_if(
                    |level_up_screen_widgets: Option<Res<LevelUpScreenWidgets>>,
                     mut run_on_next_frame: Local<bool>| {
                        if *run_on_next_frame {
                            *run_on_next_frame = false;
                            return true;
                        }

                        if level_up_screen_widgets.is_some()
                            && level_up_screen_widgets.unwrap().is_changed()
                        {
                            *run_on_next_frame = true;
                        }
                        false
                    },
                ),
                update_offered_perks.run_if(
                    |level_up_screen_state: Option<Res<LevelUpScreenState>>,
                     level_up_screen_widgets: Option<Res<LevelUpScreenWidgets>>,
                     perk_registry: Res<PerkRegistry>| {
                        if level_up_screen_state.is_none() || level_up_screen_widgets.is_none() {
                            return false;
                        }

                        let level_up_screen_state = level_up_screen_state.unwrap();
                        let level_up_screen_widgets = level_up_screen_widgets.unwrap();

                        level_up_screen_state.is_added()
                            || level_up_screen_state.is_changed()
                            || level_up_screen_widgets.is_added()
                            || perk_registry.is_changed()
                    },
                ),
                update_balance_text.run_if(|balance: Res<Balance>| balance.is_changed()),
                update_reroll_button.run_if(
                    |balance: Res<Balance>,
                     level_up_screen_configuration: Res<LevelUpScreenConfiguration>| {
                        level_up_screen_configuration.is_changed()
                            || balance.is_changed()
                    },
                ),
            ),
        );
        app.add_systems(
            Update,
            reroll_perks.run_if(
                |app_state: Res<State<AppState>>,
                 game_state: Res<State<GameState>>,
                 level_up_screen_configuration: Res<LevelUpScreenConfiguration>,
                 level_up_screen_state: Option<Res<LevelUpScreenState>>| {
                    if *app_state.get() != AppState::Game {
                        return false;
                    }

                    if *game_state.get() != GameState::LevelUpScreen {
                        return false;
                    }

                    match level_up_screen_state {
                        Some(level_up_screen_state) => {
                            level_up_screen_state.offered_perk_ids.len()
                                != (level_up_screen_configuration.number_of_perks as usize)
                        },
                        None => true,
                    }
                },
            ),
        );
        app.add_systems(Update, navigation.in_set(LevelUpScreenSystems));
        app.add_systems(
            PostUpdate,
            (select_button_interaction, reroll_button_interaction).in_set(LevelUpScreenSystems),
        );
        app.add_systems(OnExit(GameState::LevelUpScreen), despawn_level_up_screen);
        app.add_systems(OnEnter(GameState::Over), reset_level_up_screen_configuration);
        app.add_systems(
            OnEnter(GameState::Restart),
            (despawn_level_up_screen, reset_level_up_screen_configuration)
                .in_set(RestartSystems::LevelUpScreen),
        );
        app.add_systems(
            OnExit(AppState::Game),
            (despawn_level_up_screen, reset_level_up_screen_configuration),
        );
    }
}
