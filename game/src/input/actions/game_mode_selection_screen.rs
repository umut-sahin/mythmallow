use crate::prelude::*;

/// Actions that can be performed in the game mode selection screen.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum GameModeSelectionScreenAction {
    Back,
    Up,
    Down,
    Select,
}

impl GameModeSelectionScreenAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<GameModeSelectionScreenAction>::default());

        // Create the input map.
        let mut input_map = InputMap::new([
            (GameModeSelectionScreenAction::Back, KeyCode::Escape),
            (GameModeSelectionScreenAction::Select, KeyCode::Enter),
        ]);

        // Extend the input map from key bindings.
        let key_bindings = app.world.resource::<Persistent<KeyBindings>>();
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(GameModeSelectionScreenAction::Up, key_code);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(GameModeSelectionScreenAction::Down, key_code);
        }

        // Insert the input map resource.
        app.insert_resource(input_map);
    }
}
