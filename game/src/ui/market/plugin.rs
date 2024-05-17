use crate::{
    prelude::*,
    ui::market::systems::*,
};

/// Plugin for managing the market.
pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Market>();
        app.register_type::<MarketHeaderContainer>();
        app.register_type::<MarketBalanceContainer>();
        app.register_type::<MarketBalanceText>();
        app.register_type::<MarketRefreshButton>();
        app.register_type::<MarketItemsContainer>();
        app.register_type::<MarketItemContainer>();
        app.register_type::<MarketItemDetails>();
        app.register_type::<MarketItemNameText>();
        app.register_type::<MarketBuyButton>();
        app.register_type::<MarketLockButton>();
        app.register_type::<MarketContinueButton>();

        // Add systems.
        app.add_systems(OnEnter(GameState::Market), spawn_market);
        app.add_systems(
            PreUpdate,
            (
                update_market_widget_hierarchy.run_if(
                    |market_widgets: Option<Res<MarketWidgets>>,
                     mut run_on_next_frame: Local<bool>| {
                        if *run_on_next_frame {
                            *run_on_next_frame = false;
                            return true;
                        }

                        if market_widgets.is_some() && market_widgets.unwrap().is_changed() {
                            *run_on_next_frame = true;
                        }
                        false
                    },
                ),
                update_balance_text.run_if(
                    |player_query: Query<&Player, Changed<Experience>>,
                     market_spending: Res<MarketSpending>| {
                        market_spending.is_changed() || !player_query.is_empty()
                    },
                ),
                update_refresh_button.run_if(
                    |player_query: Query<&Player, Changed<Experience>>,
                     market_spending: Res<MarketSpending>,
                     market_configuration: Res<MarketConfiguration>,
                     market_state: Res<MarketState>| {
                        market_state.is_changed()
                            || market_configuration.is_changed()
                            || market_spending.is_changed()
                            || !player_query.is_empty()
                    },
                ),
            ),
        );
        app.add_systems(
            PreUpdate,
            update_offered_items.run_if(
                |player_query: Query<&Player, Changed<Experience>>,
                 market_spending: Res<MarketSpending>,
                 market_state: Res<MarketState>,
                 market_widgets: Option<Res<MarketWidgets>>,
                 item_registry: Res<ItemRegistry>| {
                    let balance_changed = market_spending.is_changed() || !player_query.is_empty();
                    market_widgets.is_some()
                        && (market_widgets.unwrap().is_added()
                            || market_state.is_changed()
                            || balance_changed
                            || item_registry.is_changed())
                },
            ),
        );
        app.add_systems(Update, navigation.in_set(MarketSystems));
        app.add_systems(
            PostUpdate,
            (
                buy_button_interaction,
                lock_button_interaction,
                refresh_button_interaction,
                continue_button_interaction,
            )
                .in_set(MarketSystems),
        );
        app.add_systems(OnExit(GameState::Market), despawn_market);
    }
}
