use crate::prelude::*;

/// Actions that can be performed in the level up screen.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum LevelUpScreenAction {
    Pause,
    Up,
    Down,
    Left,
    Right,
    Select,
}

impl LevelUpScreenAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<LevelUpScreenAction>::default());

        // Create the input map.
        let mut input_map = InputMap::new([(LevelUpScreenAction::Select, KeyCode::Enter)]);

        // Extend the input map from key bindings.
        let key_bindings = app.world.resource::<Persistent<KeyBindings>>();
        for key_code in key_bindings.pause.iter().cloned() {
            input_map.insert(LevelUpScreenAction::Pause, key_code);
        }
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(LevelUpScreenAction::Up, key_code);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(LevelUpScreenAction::Down, key_code);
        }
        for key_code in key_bindings.left.iter().cloned() {
            input_map.insert(LevelUpScreenAction::Left, key_code);
        }
        for key_code in key_bindings.right.iter().cloned() {
            input_map.insert(LevelUpScreenAction::Right, key_code);
        }

        // Insert the input map resource.
        app.insert_resource(input_map);
    }
}
