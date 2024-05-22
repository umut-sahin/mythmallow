use crate::prelude::*;


/// Gets the style of the current wave container in the HUD.
pub fn current_wave_container() -> Style {
    Style {
        position_type: PositionType::Absolute,
        align_self: AlignSelf::End,
        justify_self: JustifySelf::Start,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(10.00),
        height: Val::Percent(5.00),
        top: Val::Percent(4.00),
        ..default()
    }
}

/// Gets the style of the remaining seconds container in the HUD.
pub fn remaining_seconds_container() -> Style {
    Style {
        position_type: PositionType::Absolute,
        align_self: AlignSelf::End,
        justify_self: JustifySelf::Start,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(10.00),
        height: Val::Percent(5.00),
        top: Val::Percent(8.00),
        ..default()
    }
}
