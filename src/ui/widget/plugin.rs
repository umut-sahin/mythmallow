use crate::{
    prelude::*,
    ui::widget::systems::*,
};

/// Plugin for simplifying user interface management.
pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Widget>();
        app.register_type::<WidgetColors>();
        app.register_type::<WidgetSelected>();
        app.register_type::<WidgetUp>();
        app.register_type::<WidgetDown>();

        app.add_systems(
            PostUpdate,
            (
                manage_widget_selected_on_mouse_wiggle,
                set_is_selected_when_widget_selected_component_is_added,
                ensure_single_widget_is_selected,
                remove_widget_selected_from_widgets_which_are_not_selected,
                update_widget_state_on_user_interactions,
                update_widget_colors_on_state_change,
            )
                .chain()
                .in_set(MenuSystems),
        );
    }
}
