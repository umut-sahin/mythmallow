use crate::{
    prelude::*,
    ui::game_mode_selection_screen::constants::*,
};


/// Gets the style of the buttons in the game mode selection screen.
pub fn button() -> Style {
    Style {
        width: Val::Px(BUTTON_WIDTH),
        height: Val::Px(BUTTON_HEIGHT),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
