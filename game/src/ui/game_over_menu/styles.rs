use crate::{
    prelude::*,
    ui::game_over_menu::constants::*,
};


/// Gets the style of the title in the game over menu.
pub fn title() -> Style {
    Style { justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() }
}


/// Gets the style of the buttons in the game over menu.
pub fn button() -> Style {
    Style {
        width: Val::Px(BUTTON_WIDTH),
        height: Val::Px(BUTTON_HEIGHT),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
