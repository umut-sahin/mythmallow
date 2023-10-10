use crate::prelude::*;

/// Plugin for managing configurations.
pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Args>();
        app.register_type::<KeyBindings>();

        KeyBindings::initialize(app);

        let args = app.world.resource::<Args>();
        if args.start_in_game {
            app.world.insert_resource(NextState(Some(AppState::Game)));
        }
    }
}
