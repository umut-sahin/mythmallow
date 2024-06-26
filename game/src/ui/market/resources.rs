use crate::prelude::*;


/// Resource for the widgets of the market.
#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct MarketWidgets(
    /// Rows of widgets in the market.
    ///
    /// - Balance & Refresh button
    /// - Buy buttons
    /// - Lock buttons
    /// - Continue button
    pub [Vec<Entity>; 4],
);


/// Resource for the previously selected widget in the market.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
pub struct PreviouslySelectedMarketWidget(pub Entity);
