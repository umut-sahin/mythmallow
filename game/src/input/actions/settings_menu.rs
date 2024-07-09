use crate::prelude::*;

/// Actions that can be performed in the settings menu.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum SettingsMenuAction {
    Back,
    Up,
    Down,
    Left,
    Right,
    Select,
}

impl SettingsMenuAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<SettingsMenuAction>::default());

        // Create the input map.
        let mut input_map = InputMap::new([
            (SettingsMenuAction::Back, KeyCode::Escape),
            (SettingsMenuAction::Select, KeyCode::Enter),
        ]);

        // Extend the input map from key bindings.
        let key_bindings = app.world_mut().resource::<Persistent<KeyBindings>>();
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(SettingsMenuAction::Up, key_code);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(SettingsMenuAction::Down, key_code);
        }
        for key_code in key_bindings.left.iter().cloned() {
            input_map.insert(SettingsMenuAction::Left, key_code);
        }
        for key_code in key_bindings.right.iter().cloned() {
            input_map.insert(SettingsMenuAction::Right, key_code);
        }

        // Insert the input map resource.
        app.insert_resource(input_map);
    }
}
