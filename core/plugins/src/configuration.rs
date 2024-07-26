use {
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
};

/// Plugin for managing the configuration of the application.
pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<Arguments>();

        // Initialize resources.
        Arguments::initialize(app);
    }
}
