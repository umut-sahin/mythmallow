use crate::prelude::*;


/// Updates all localized texts.
pub fn update_all_localized_texts(
    mut localized_text_query: Query<(&mut Text, &LocalizedText)>,
    localization: Res<Localization>,
) {
    for (mut text, localized_text) in localized_text_query.iter_mut() {
        text.sections[0].value = localized_text.get(&localization).to_string();
    }
}

/// Updates changed localized texts.
pub fn update_changed_localized_texts(
    mut localized_text_query: Query<(&mut Text, &LocalizedText), Changed<LocalizedText>>,
    localization: Res<Localization>,
) {
    for (mut text, localized_text) in localized_text_query.iter_mut() {
        text.sections[0].value = localized_text.get(&localization).to_string();
    }
}


/// Manages widget selected component on mouse wiggle.
pub fn manage_widget_selected_on_mouse_wiggle(
    mut commands: Commands,
    widget_query: Query<(Entity, &Interaction), With<Widget>>,
    mouse_motion_reader: EventReader<MouseMotion>,
) {
    if !mouse_motion_reader.is_empty() {
        let mut hovered = false;
        for (entity, interaction) in &widget_query {
            if *interaction == Interaction::Hovered {
                commands.entity(entity).insert(WidgetSelected::now());
                hovered = true;
                break;
            }
        }
        if !hovered {
            for (entity, interaction) in &widget_query {
                if *interaction == Interaction::Pressed {
                    commands.entity(entity).insert(WidgetSelected::now());
                    break;
                }
            }
        }
    }
}

/// Sets is selected property of widgets when widget selected component is added.
pub fn set_is_selected_when_widget_selected_component_is_added(
    mut widget_query: Query<&mut Widget, Added<WidgetSelected>>,
) {
    for mut widget in &mut widget_query {
        widget.is_selected = true;
    }
}

/// Ensures that there is a single widget with is selected property set to true.
pub fn ensure_single_widget_is_selected(
    mut selected_widgets_query: Query<(&mut Widget, &WidgetSelected)>,
) {
    let mut selected_widgets = selected_widgets_query.iter_mut().collect::<Vec<_>>();
    selected_widgets.sort_by(|(_, lhs_selected), (_, rhs_selected)| {
        lhs_selected.at.cmp(&rhs_selected.at).reverse()
    });
    for (ref mut widget, _) in selected_widgets.iter_mut().skip(1) {
        widget.is_selected = false;
    }
}

/// Removes widget selected component from the widgets without their selected property set to true.
pub fn remove_widget_selected_from_widgets_which_are_not_selected(
    mut commands: Commands,
    widget_query: Query<(Entity, &Widget), (Changed<Widget>, With<WidgetSelected>)>,
) {
    for (entity, widget) in &widget_query {
        if !widget.is_selected {
            commands.entity(entity).remove::<WidgetSelected>();
        }
    }
}

/// Updates widget state on user interactions.
pub fn update_widget_state_on_user_interactions(
    mut commands: Commands,
    mut widget_query: Query<(Entity, &mut Widget, &Interaction), Changed<Interaction>>,
) {
    for (entity, mut widget, interaction) in &mut widget_query {
        match interaction {
            Interaction::None => {
                widget.is_hovered = false;
                widget.is_pressed = false;
            },
            Interaction::Hovered => {
                if widget.is_pressed {
                    widget.clicked = true;
                }

                commands.entity(entity).insert(WidgetSelected::now());

                widget.is_selected = true;
                widget.is_hovered = true;
                widget.is_pressed = false;
            },
            Interaction::Pressed => {
                widget.is_selected = true;
                widget.is_hovered = true;
                widget.is_pressed = true;
            },
        }
    }
}

/// Updates colors of the widgets with changed states.
pub fn update_widget_colors_on_state_change(
    mut widget_query: Query<(&Widget, &WidgetColors, &mut BackgroundColor), Changed<Widget>>,
) {
    for (widget, widget_colors, mut background_color) in &mut widget_query {
        let new_background_color = if widget.is_pressed {
            widget_colors.pressed.into()
        } else if widget.is_selected {
            widget_colors.selected.into()
        } else {
            widget_colors.normal.into()
        };
        *background_color = new_background_color;
    }
}


/// Changes the text color of the widget to disabled text color when widget is disabled.
pub fn change_text_color_when_disabled(
    mut text_query: Query<(&Parent, &mut Text)>,
    widget_query: Query<&WidgetColors, (With<Widget>, Added<WidgetDisabled>)>,
) {
    for (parent, mut text) in text_query.iter_mut() {
        if let Ok(widget_colors) = widget_query.get(parent.get()) {
            text.sections[0].style.color = widget_colors.disabled_text;
        }
    }
}

/// Changes the text color of the widget to regular text color when widget is enabled.
pub fn change_text_color_when_enabled(
    mut removed_components: RemovedComponents<WidgetDisabled>,
    mut text_query: Query<(&Parent, &mut Text)>,
    widget_query: Query<&WidgetColors, With<Widget>>,
) {
    for widget_entity in removed_components.read() {
        let widget_colors = match widget_query.get(widget_entity) {
            Ok(query_result) => query_result,
            Err(_) => continue,
        };
        for (parent, mut text) in text_query.iter_mut() {
            if parent.get() == widget_entity {
                text.sections[0].style.color = widget_colors.text;
                break;
            }
        }
    }
}
