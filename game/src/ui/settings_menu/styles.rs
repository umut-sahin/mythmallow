use crate::prelude::*;


/// Gets the style of the root container of the settings menu.
pub fn root() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Percent(1.50),
        ..default()
    }
}


/// Gets the style of the language setting container of the settings menu.
pub fn language_setting_container() -> Style {
    Style {
        width: Val::Percent(80.0),
        height: Val::Percent(25.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        column_gap: Val::Percent(1.00),
        ..default()
    }
}

/// Gets the style of language setting changers of the settings menu.
pub fn language_setting_changer() -> Style {
    Style {
        width: Val::Percent(4.50),
        height: Val::Percent(25.00),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::horizontal(Val::Percent(2.50)),
        ..default()
    }
}

/// Gets the style of the language setting name of the settings menu.
pub fn language_setting_name() -> Style {
    Style { ..default() }
}

/// Gets the style of the language setting value of the settings menu.
pub fn language_setting_value() -> Style {
    Style { ..default() }
}


/// Gets the style of the back button in the settings menu.
pub fn back_button() -> Style {
    Style {
        width: Val::Percent(16.00),
        height: Val::Percent(9.00),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
