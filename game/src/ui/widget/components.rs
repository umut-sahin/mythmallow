use crate::{
    prelude::*,
    ui::widget::constants::*,
};


/// Component for the widgets in the user interface.
#[derive(Component, Debug, Reflect)]
pub struct Widget {
    /// Whether the widget is selected.
    pub is_selected: bool,
    /// Whether the widget is hovered with the mouse.
    pub is_hovered: bool,
    /// Whether the widget is pressed with the mouse.
    pub is_pressed: bool,
    /// Whether the widget is just clicked.
    pub clicked: bool,
}

impl Widget {
    /// Makes the widget selected.
    pub fn selected(mut self) -> Self {
        self.is_selected = true;
        self
    }
}

impl Widget {
    /// Spawns a button widget.
    pub fn button(
        commands: &mut Commands,
        bundle: impl Bundle,
        style: &Style,
        color: WidgetColors,
        font: &Handle<Font>,
        size: f32,
        text: impl AsRef<str>,
    ) -> Entity {
        commands
            .spawn((
                bundle,
                ButtonBundle {
                    style: style.clone(),
                    background_color: color.normal.into(),
                    ..default()
                },
                color,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("Text"),
                    TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                text.as_ref(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: size,
                                    color: color.text,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    },
                ));
            })
            .id()
    }
}

impl Widget {
    /// Performs an action if the button is just clicked.
    ///
    /// It clears the clicked status of the widget.
    /// So if it is called multiple times in a frame,
    /// only the action in the first call will be performed.
    pub fn on_click(&mut self, action: impl FnOnce()) {
        if self.clicked {
            self.clicked = false;
            action()
        }
    }
}

impl Default for Widget {
    fn default() -> Widget {
        Widget { is_selected: false, is_hovered: false, is_pressed: false, clicked: false }
    }
}


/// Component for the colors of the widgets in the user interface.
#[derive(Clone, Copy, Component, Debug, Reflect)]
pub struct WidgetColors {
    /// Color of the text within the widget.
    pub text: Color,
    /// Background color of the widget.
    pub normal: Color,
    /// Background color of the widget when the widget is selected.
    pub selected: Color,
    /// Background color of the widget when the widget is pressed.
    pub pressed: Color,
}

impl WidgetColors {
    /// Gets the default button widget colors.
    pub fn button() -> WidgetColors {
        DEFAULT_BUTTON_COLORS
    }
}

impl WidgetColors {
    /// Sets the text color.
    pub fn text(mut self, text: Color) -> Self {
        self.text = text;
        self
    }

    /// Sets the normal color.
    pub fn normal(mut self, normal: Color) -> Self {
        self.normal = normal;
        self
    }

    /// Sets the selected color.
    pub fn selected(mut self, selected: Color) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the pressed color.
    pub fn pressed(mut self, pressed: Color) -> Self {
        self.pressed = pressed;
        self
    }
}


/// Tag component for the selected widget.
#[derive(Component, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub struct WidgetSelected {
    /// The instant at which the widget became selected.
    pub at: Instant,
}

impl WidgetSelected {
    /// Creates a new widget selected component at this instant.
    pub fn now() -> WidgetSelected {
        WidgetSelected { at: Instant::now() }
    }
}


/// Component for indicating the widget up of the attached widget.
#[derive(Component, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub struct WidgetUp(pub Entity);


/// Component for indicating the widget down of the attached widget.
#[derive(Component, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub struct WidgetDown(pub Entity);
