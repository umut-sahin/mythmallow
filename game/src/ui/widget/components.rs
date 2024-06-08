use crate::{
    prelude::*,
    ui::widget::constants::*,
};


/// Component for localized texts.
#[derive(Clone, Component, Debug)]
pub enum LocalizedText {
    Localized {
        key: &'static str,
        args: SmallVec<[(&'static str, SmolStr); 1]>,
        fallback: Cow<'static, str>,
    },
    Constant {
        text: Cow<'static, str>,
    },
}

impl LocalizedText {
    /// Gets the localized text.
    pub fn get(&self, localization: &Localization) -> Cow<'static, str> {
        match self {
            LocalizedText::Localized { key, args, fallback } => {
                let mut fluent_args = FluentArgs::new();
                for (key, value) in args {
                    fluent_args.set(*key, value.as_str());
                }

                let request = Request::new(key).args(&fluent_args);
                localization.content(request).map(Cow::Owned).unwrap_or_else(|| fallback.clone())
            },
            LocalizedText::Constant { text } => text.clone(),
        }
    }

    /// Gets the fallback of the localized text.
    pub fn fallback(&self) -> Cow<'static, str> {
        match self {
            LocalizedText::Localized { fallback, .. } => fallback.clone(),
            LocalizedText::Constant { text } => text.clone(),
        }
    }
}

impl<T: ToString> From<T> for LocalizedText {
    fn from(value: T) -> LocalizedText {
        LocalizedText::Constant { text: Cow::Owned(value.to_string()) }
    }
}


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
    /// Spawns a container widget.
    pub fn container(
        commands: &mut Commands,
        bundle: impl Bundle,
        style: &Style,
        color: WidgetColors,
    ) -> Entity {
        commands
            .spawn((
                bundle,
                NodeBundle {
                    style: style.clone(),
                    background_color: color.normal.into(),
                    ..default()
                },
                Interaction::None,
                color,
            ))
            .id()
    }

    /// Spawns a button widget.
    pub fn button(
        commands: &mut Commands,
        bundle: impl Bundle,
        style: &Style,
        color: WidgetColors,
        font: &Handle<Font>,
        font_size: f32,
        text: impl Into<LocalizedText>,
        localization: &Localization,
    ) -> Entity {
        let text = text.into();
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
                                text.get(localization),
                                TextStyle { font: font.clone(), font_size, color: color.text },
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    },
                    text,
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
    /// Color of the text within the widget when the widget is disabled.
    pub disabled_text: Color,
    /// Background color of the widget.
    pub normal: Color,
    /// Background color of the widget when the widget is selected.
    pub selected: Color,
    /// Background color of the widget when the widget is pressed.
    pub pressed: Color,
}

impl WidgetColors {
    /// Gets the default container widget colors.
    pub fn container() -> WidgetColors {
        DEFAULT_CONTAINER_COLORS
    }

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


/// Tag component for disabled widgets.
#[derive(Component, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub struct WidgetDisabled;


/// Tag component for the selected widget.
#[derive(Component, Debug)]
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


/// Component for indicating the widget left of the attached widget.
#[derive(Component, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub struct WidgetLeft(pub Entity);


/// Component for indicating the widget right of the attached widget.
#[derive(Component, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub struct WidgetRight(pub Entity);
