use crate::prelude::*;

/// Plugin for managing the configuration of the application.
pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<GeneralSettings>();
        app.register_type::<KeyBindings>();

        // Initialize configurations.
        GeneralSettings::initialize(app);
        KeyBindings::initialize(app);
    }
}
