use crate::prelude::*;


/// Gets the style of the buttons in the game mode selection screen.
pub fn button() -> Style {
    Style {
        width: Val::Percent(16.00),
        height: Val::Percent(9.00),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
