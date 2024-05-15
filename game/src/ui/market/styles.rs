use crate::{
    prelude::*,
    ui::market::constants::*,
};


/// Gets the style of the header container in the market.
pub fn header_container() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(HEADER_CONTAINER_WIDTH_PERCENT),
        height: Val::Percent(HEADER_CONTAINER_HEIGHT_PERCENT),
        ..default()
    }
}

/// Gets the style of the balance container in the market.
pub fn balance_container() -> Style {
    Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Start,
        width: Val::Percent(BALANCE_CONTAINER_WIDTH_PERCENT),
        height: Val::Percent(BALANCE_CONTAINER_HEIGHT_PERCENT),
        ..default()
    }
}

/// Gets the style of the balance text in the market.
pub fn balance_text() -> Style {
    Style { ..default() }
}

/// Gets the style of the refresh button in the market.
pub fn refresh_button() -> Style {
    Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(REFRESH_BUTTON_WIDTH_PERCENT),
        height: Val::Percent(REFRESH_BUTTON_HEIGHT_PERCENT),
        ..default()
    }
}


/// Gets the style of the items container in the market.
pub fn items_container() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Percent(ITEMS_CONTAINER_COLUMN_GAP_PERCENT),
        width: Val::Percent(ITEMS_CONTAINER_WIDTH_PERCENT),
        height: Val::Percent(ITEMS_CONTAINER_HEIGHT_PERCENT),
        ..default()
    }
}

/// Gets the style of item containers in the market.
pub fn item_container() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Percent(ITEM_CONTAINER_ROW_GAP_PERCENT),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceEvenly,
        width: Val::Percent(ITEM_CONTAINER_WIDTH_PERCENT),
        height: Val::Percent(ITEM_CONTAINER_HEIGHT_PERCENT),
        ..default()
    }
}

/// Gets the style of item details in the market.
pub fn item_details() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Percent(ITEM_DETAILS_ROW_GAP_PERCENT),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(ITEM_DETAILS_WIDTH_PERCENT),
        height: Val::Percent(ITEM_DETAILS_HEIGHT_PERCENT),
        ..default()
    }
}

/// Gets the style of item name texts in the market.
pub fn item_name_text() -> Style {
    Style { ..default() }
}

/// Gets the style of buy buttons in the market.
pub fn buy_button() -> Style {
    Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        justify_self: JustifySelf::End,
        width: Val::Percent(BUY_BUTTON_WIDTH_PERCENT),
        height: Val::Percent(BUY_BUTTON_HEIGHT_PERCENT),
        ..default()
    }
}

/// Gets the style of lock buttons in the market.
pub fn lock_button() -> Style {
    Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(LOCK_BUTTON_WIDTH_PERCENT),
        height: Val::Percent(LOCK_BUTTON_HEIGHT_PERCENT),
        ..default()
    }
}


/// Gets the style of the continue button in the market.
pub fn continue_button() -> Style {
    Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(CONTINUE_BUTTON_WIDTH_PERCENT),
        height: Val::Percent(CONTINUE_BUTTON_HEIGHT_PERCENT),
        ..default()
    }
}
