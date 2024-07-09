use crate::prelude::*;


/// Gets the style of the HUD.
pub fn hud() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Percent(1.00),
        width: Val::Percent(100.00),
        height: Val::Percent(100.00),
        ..default()
    }
}


/// Gets the style of the health bar in the HUD.
pub fn health_bar() -> Style {
    Style {
        position_type: PositionType::Absolute,
        align_self: AlignSelf::Start,
        justify_self: JustifySelf::Start,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(15.00),
        height: Val::Percent(5.00),
        top: Val::Percent(4.00),
        left: Val::Percent(1.50),
        ..default()
    }
}

/// Gets the style of the experience bar in the HUD.
pub fn experience_bar() -> Style {
    Style {
        position_type: PositionType::Absolute,
        align_self: AlignSelf::Start,
        justify_self: JustifySelf::Start,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(15.00),
        height: Val::Percent(5.00),
        top: Val::Percent(9.50),
        left: Val::Percent(1.50),
        ..default()
    }
}

/// Gets the style of the balance container in the HUD.
pub fn balance_container() -> Style {
    Style {
        position_type: PositionType::Absolute,
        align_self: AlignSelf::Start,
        justify_self: JustifySelf::Start,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(15.00),
        height: Val::Percent(5.00),
        top: Val::Percent(14.00),
        left: Val::Percent(1.50),
        ..default()
    }
}
