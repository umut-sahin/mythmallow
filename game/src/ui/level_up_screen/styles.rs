use crate::prelude::*;


/// Gets the style of the root container in the level up screen.
pub fn level_up_screen() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        justify_items: JustifyItems::Center,
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Percent(7.00),
        ..default()
    }
}


/// Gets the style of the perks container in the level up screen.
pub fn perks_container() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Percent(3.00),
        width: Val::Percent(80.00),
        height: Val::Percent(50.00),
        ..default()
    }
}

/// Gets the style of perk containers in the level up screen.
pub fn perk_container() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Percent(3.00),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceEvenly,
        width: Val::Percent(100.00),
        height: Val::Percent(100.00),
        ..default()
    }
}

/// Gets the style of perk details in the level up screen.
pub fn perk_details() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Percent(15.00),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(100.00),
        height: Val::Percent(100.00),
        ..default()
    }
}

/// Gets the style of perk name texts in the level up screen.
pub fn perk_name_text() -> Style {
    Style { ..default() }
}

/// Gets the style of perk description texts in the level up screen.
pub fn perk_description_text() -> Style {
    Style { ..default() }
}

/// Gets the style of select buttons in the level up screen.
pub fn select_button() -> Style {
    Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(40.00),
        height: Val::Percent(16.00),
        ..default()
    }
}


/// Gets the style of the footer container in the level up screen.
pub fn footer_container() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(80.00),
        height: Val::Percent(10.00),
        ..default()
    }
}

/// Gets the style of the balance container in the level up screen.
pub fn balance_container() -> Style {
    Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Start,
        width: Val::Percent(20.00),
        height: Val::Percent(90.00),
        padding: UiRect::left(Val::Percent(0.50)),
        ..default()
    }
}

/// Gets the style of the balance text in the level up screen.
pub fn balance_text() -> Style {
    Style { ..default() }
}

/// Gets the style of the reroll button in the level up screen.
pub fn reroll_button() -> Style {
    Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(25.00),
        height: Val::Percent(90.00),
        ..default()
    }
}
