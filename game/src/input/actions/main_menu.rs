use crate::prelude::*;

/// Actions that can be performed in the main menu.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum MainMenuAction {
    Up,
    Down,
    Select,
}

impl MainMenuAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<MainMenuAction>::default());

        // Create the input map.
        let mut input_map = InputMap::new([(MainMenuAction::Select, KeyCode::Enter)]);

        // Extend the input map from key bindings.
        let key_bindings = app.world_mut().resource::<Persistent<KeyBindings>>();
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(MainMenuAction::Up, key_code);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(MainMenuAction::Down, key_code);
        }

        // Insert the input map resource.
        app.insert_resource(input_map);
    }
}
