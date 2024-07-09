use crate::prelude::*;


/// Default colors of the containers in the user interface.
pub const DEFAULT_CONTAINER_COLORS: WidgetColors = WidgetColors {
    text: Color::srgba(1.00, 1.00, 1.00, 1.00),
    disabled_text: Color::srgba(0.86, 0.12, 0.11, 1.00),
    normal: Color::srgba(0.15, 0.15, 0.15, 0.00),
    selected: Color::srgba(0.25, 0.25, 0.25, 0.00),
    pressed: Color::srgba(0.35, 0.35, 0.35, 0.00),
};


/// Default colors of the buttons in the user interface.
pub const DEFAULT_BUTTON_COLORS: WidgetColors = WidgetColors {
    text: Color::srgb(1.00, 1.00, 1.00),
    disabled_text: Color::srgba(0.86, 0.12, 0.11, 1.00),
    normal: Color::srgb(0.15, 0.15, 0.15),
    selected: Color::srgb(0.25, 0.25, 0.25),
    pressed: Color::srgb(0.35, 0.35, 0.35),
};
