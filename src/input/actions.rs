use crate::prelude::*;


/// Actions that can be performed anywhere.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum GlobalAction {
    ToggleFullscreen,
}

impl GlobalAction {
    /// Sets up the app for the action.
    pub fn setup(app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default());

        let input_map = InputMap::new([(KeyCode::F11, GlobalAction::ToggleFullscreen)]);

        app.insert_resource(input_map);
        app.insert_resource(ActionState::<GlobalAction>::default());
    }
}


/// Actions that can be performed in the main menu.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum MainMenuAction {
    Up,
    Select,
    Down,
}

impl MainMenuAction {
    /// Sets up the app for the action.
    pub fn setup(app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MainMenuAction>::default());

        let key_bindings = app.world.resource::<Persistent<KeyBindings>>();

        let mut input_map = InputMap::new([(KeyCode::Return, MainMenuAction::Select)]);
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(key_code, MainMenuAction::Up);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(key_code, MainMenuAction::Down);
        }

        app.insert_resource(input_map);
    }
}


/// Actions that can be performed in the game.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum GameAction {
    MoveUp,
    MoveLeft,
    MoveDown,
    MoveRight,
    Dash,
    Pause,
}

impl GameAction {
    /// Sets up the app for the action.
    pub fn setup(app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GameAction>::default());

        let key_bindings = app.world.resource::<Persistent<KeyBindings>>();

        let mut input_map = InputMap::default();
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

        app.insert_resource(input_map);
    }
}


/// Actions that can be performed in the pause menu.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum PauseMenuAction {
    Resume,
    Up,
    Select,
    Down,
}

impl PauseMenuAction {
    /// Sets up the app for the action.
    pub fn setup(app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PauseMenuAction>::default());

        let key_bindings = app.world.resource::<Persistent<KeyBindings>>();

        let mut input_map = InputMap::new([
            (KeyCode::Escape, PauseMenuAction::Resume),
            (KeyCode::Return, PauseMenuAction::Select),
        ]);
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(key_code, PauseMenuAction::Up);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(key_code, PauseMenuAction::Down);
        }

        app.insert_resource(input_map);
    }
}
