use crate::{
    prelude::*,
    ui::pause_menu::constants::*,
};


/// Gets the style of buttons in the pause menu.
pub fn button() -> Style {
    Style {
        width: Val::Px(BUTTON_WIDTH),
        height: Val::Px(BUTTON_HEIGHT),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
