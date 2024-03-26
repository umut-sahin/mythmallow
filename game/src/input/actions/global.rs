use crate::prelude::*;

/// Actions that can be performed anywhere.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum GlobalAction {
    ToggleFullscreen,
    ToggleDiagnosticsOverlay,

    #[cfg(feature = "development")]
    TogglePhysicsDebug,
}

impl GlobalAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default());

        // Create the input map.
        let mut input_map = InputMap::default();

        input_map.insert(GlobalAction::ToggleFullscreen, KeyCode::F11);
        input_map.insert(GlobalAction::ToggleDiagnosticsOverlay, KeyCode::F10);

        #[cfg(feature = "development")]
        input_map.insert(
            GlobalAction::TogglePhysicsDebug,
            UserInput::chord([KeyCode::ControlLeft, KeyCode::KeyP]),
        );

        // Insert the input map resource.
        app.insert_resource(input_map);

        // Insert the global action state as a resource.
        app.insert_resource(ActionState::<GlobalAction>::default());
    }
}
