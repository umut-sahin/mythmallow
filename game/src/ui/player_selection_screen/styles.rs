use crate::prelude::*;


/// Gets the style of the root container of the player selection screen.
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


/// Gets the style of the buttons in the player selection screen.
pub fn button() -> Style {
    Style {
        width: Val::Percent(16.00),
        height: Val::Percent(9.00),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
