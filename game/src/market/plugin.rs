use crate::{
    market::{
        commands::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing the market.
pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<Balance>();
        app.register_type::<ExperienceToBalanceRatio>();
        app.register_type::<MarketConfiguration>();

        // Insert resources.
        app.init_resource::<Balance>();
        app.init_resource::<ExperienceToBalanceRatio>();
        app.init_resource::<MarketConfiguration>();
        app.init_resource::<MarketState>();

        // Add console commands.
        app.add_console_command::<MarketCommand, _>(apply_market_command);

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            set_balance_and_free_refreshes.in_set(InitializationSystems::Market),
        );
        app.add_systems(PreUpdate, gain_balance);
        app.add_systems(PreUpdate, process_acquirements.run_if(in_state(AppState::Game)));
        app.add_systems(
            Update,
            refresh_market_automatically.run_if(
                |app_state: Res<State<AppState>>,
                 game_state: Res<State<GameState>>,
                 market_state: Res<MarketState>,
                 market_configuration: Res<MarketConfiguration>| {
                    if *app_state.get() != AppState::Game {
                        return false;
                    }

                    match game_state.get() {
                        GameState::Market => {},
                        GameState::Playing | GameState::Paused => {
                            if market_state.offered_item_ids.is_empty() {
                                return false;
                            }
                        },
                        _ => return false,
                    }

                    market_state.offered_item_ids.len()
                        != (market_configuration.number_of_items as usize)
                },
            ),
        );
        app.add_systems(PostUpdate, open_market.in_set(GameplaySystems::Market));
        app.add_systems(OnEnter(GameState::Over), reset_market);
        app.add_systems(OnEnter(GameState::Restart), reset_market.in_set(RestartSystems::Market));
        app.add_systems(OnExit(AppState::Game), reset_market);
    }
}
