use crate::{
    prelude::*,
    ui::main_menu::constants::*,
};


/// Gets the style of the buttons in the main menu.
pub fn button() -> Style {
    Style {
        width: Val::Px(BUTTON_WIDTH),
        height: Val::Px(BUTTON_HEIGHT),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
