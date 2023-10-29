use crate::prelude::*;

/// Actions that can be performed in the game.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum GameAction {
    Pause,
    MoveUp,
    MoveLeft,
    MoveDown,
    MoveRight,
    Dash,
}

impl GameAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<GameAction>::default());

        // Create the input map.
        let mut input_map = InputMap::default();

        // Extend the input map from key bindings.
        let key_bindings = app.world.resource::<Persistent<KeyBindings>>();
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(key_code, GameAction::MoveUp);
        }
        for key_code in key_bindings.left.iter().cloned() {
            input_map.insert(key_code, GameAction::MoveLeft);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(key_code, GameAction::MoveDown);
        }
        for key_code in key_bindings.right.iter().cloned() {
            input_map.insert(key_code, GameAction::MoveRight);
        }
        for key_code in key_bindings.dash.iter().cloned() {
            input_map.insert(key_code, GameAction::Dash);
        }
        for key_code in key_bindings.pause.iter().cloned() {
            input_map.insert(key_code, GameAction::Pause);
        }

        // Insert the input map resource.
        app.insert_resource(input_map);
    }
}
