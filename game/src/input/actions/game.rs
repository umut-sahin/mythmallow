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
    OpenMarket,
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
            input_map.insert(GameAction::MoveUp, key_code);
        }
        for key_code in key_bindings.left.iter().cloned() {
            input_map.insert(GameAction::MoveLeft, key_code);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(GameAction::MoveDown, key_code);
        }
        for key_code in key_bindings.right.iter().cloned() {
            input_map.insert(GameAction::MoveRight, key_code);
        }
        for key_code in key_bindings.dash.iter().cloned() {
            input_map.insert(GameAction::Dash, key_code);
        }
        for key_code in key_bindings.pause.iter().cloned() {
            input_map.insert(GameAction::Pause, key_code);
        }
        for key_code in key_bindings.market.iter().cloned() {
            input_map.insert(GameAction::OpenMarket, key_code);
        }

        // Insert the input map resource.
        app.insert_resource(input_map);
    }
}
