use crate::prelude::*;

/// Actions that can be performed in the pause menu.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum PauseMenuAction {
    Resume,
    Up,
    Down,
    Select,
}

impl PauseMenuAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<PauseMenuAction>::default());

        // Create the input map.
        let mut input_map = InputMap::new([
            (PauseMenuAction::Resume, KeyCode::Escape),
            (PauseMenuAction::Select, KeyCode::Enter),
        ]);

        // Extend the input map from key bindings.
        let key_bindings = app.world_mut().resource::<Persistent<KeyBindings>>();
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(PauseMenuAction::Up, key_code);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(PauseMenuAction::Down, key_code);
        }

        // Insert the input map resource.
        app.insert_resource(input_map);
    }
}
