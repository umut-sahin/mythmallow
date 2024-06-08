use crate::{
    prelude::*,
    ui::widget::systems::*,
};

/// Plugin for managing the widgets in the user interface.
pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Widget>();
        app.register_type::<WidgetColors>();
        app.register_type::<WidgetDisabled>();
        app.register_type::<WidgetUp>();
        app.register_type::<WidgetDown>();
        app.register_type::<WidgetLeft>();
        app.register_type::<WidgetRight>();

        // Add systems.
        app.add_systems(OnEnter(LocalizationState::Ready), update_all_localized_texts);
        app.add_systems(
            Last,
            update_changed_localized_texts.run_if(in_state(LocalizationState::Ready)),
        );
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
        app.add_systems(
            PostUpdate,
            (change_text_color_when_disabled, change_text_color_when_enabled),
        );
    }
}
