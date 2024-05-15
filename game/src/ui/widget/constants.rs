use crate::prelude::*;


/// Default colors of the containers in the user interface.
pub const DEFAULT_CONTAINER_COLORS: WidgetColors = WidgetColors {
    text: Color::rgba(1.00, 1.00, 1.00, 1.00),
    disabled_text: Color::rgba(0.86, 0.12, 0.11, 1.00),
    normal: Color::rgba(0.15, 0.15, 0.15, 0.00),
    selected: Color::rgba(0.25, 0.25, 0.25, 0.00),
    pressed: Color::rgba(0.35, 0.35, 0.35, 0.00),
};


/// Default colors of the buttons in the user interface.
pub const DEFAULT_BUTTON_COLORS: WidgetColors = WidgetColors {
    text: Color::rgb(1.00, 1.00, 1.00),
    disabled_text: Color::rgba(0.86, 0.12, 0.11, 1.00),
    normal: Color::rgb(0.15, 0.15, 0.15),
    selected: Color::rgb(0.25, 0.25, 0.25),
    pressed: Color::rgb(0.35, 0.35, 0.35),
};
