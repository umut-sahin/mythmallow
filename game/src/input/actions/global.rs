use crate::prelude::*;

/// Actions that can be performed anywhere.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum GlobalAction {
    ToggleFullscreen,
    ToggleDiagnosticsOverlay,
}

impl GlobalAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default());

        // Create the input map.
        let input_map = InputMap::new([
            (KeyCode::F11, GlobalAction::ToggleFullscreen),
            (KeyCode::F10, GlobalAction::ToggleDiagnosticsOverlay),
        ]);

        // Insert the input map resource.
        app.insert_resource(input_map);

        // Insert the global action state as a resource.
        app.insert_resource(ActionState::<GlobalAction>::default());
    }
}
