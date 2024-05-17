use crate::prelude::*;

/// Actions that can be performed in the market.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum MarketAction {
    Pause,
    Close,
    Up,
    Down,
    Left,
    Right,
    Select,
}

impl MarketAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<MarketAction>::default());

        // Create the input map.
        let mut input_map = InputMap::new([(MarketAction::Select, KeyCode::Enter)]);

        // Extend the input map from key bindings.
        let key_bindings = app.world.resource::<Persistent<KeyBindings>>();
        for key_code in key_bindings.pause.iter().cloned() {
            input_map.insert(MarketAction::Pause, key_code);
        }
        for key_code in key_bindings.market.iter().cloned() {
            input_map.insert(MarketAction::Close, key_code);
        }
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(MarketAction::Up, key_code);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(MarketAction::Down, key_code);
        }
        for key_code in key_bindings.left.iter().cloned() {
            input_map.insert(MarketAction::Left, key_code);
        }
        for key_code in key_bindings.right.iter().cloned() {
            input_map.insert(MarketAction::Right, key_code);
        }

        // Insert the input map resource.
        app.insert_resource(input_map);
    }
}
