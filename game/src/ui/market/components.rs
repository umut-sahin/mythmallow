use crate::prelude::*;


/// Tag component for the market.
#[derive(Component, Debug, Reflect)]
pub struct Market;


/// Tag component for the header container in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketHeaderContainer;


/// Tag component for the balance container in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketBalanceContainer;


/// Tag component for the balance text in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketBalanceText;


/// Tag component for the refresh button in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketRefreshButton {
    pub cost: Experience,
}


/// Tag component for the items container in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketItemsContainer;


/// Tag component for item containers in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketItemContainer;


/// Tag component for item details in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketItemDetails;


/// Tag component for item name texts in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketItemNameText;


/// Tag component for buy buttons in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketBuyButton {
    pub item_index: usize,
    pub price: Experience,
}


/// Tag component for lock buttons in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketLockButton {
    pub item_index: usize,
}


/// Tag component for the continue button in the market.
#[derive(Component, Debug, Reflect)]
pub struct MarketContinueButton;
