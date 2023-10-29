use {
    crate::{
        bident_of_hades::BidentOfHadesPlugin,
        bow_of_artemis::BowOfArtemisPlugin,
    },
    mythmallow::prelude::*,
};

/// Plugin for managing the items from Greek mythology.
pub struct GreekItemsPlugin;

impl Plugin for GreekItemsPlugin {
    fn build(&self, app: &mut App) {
        // Add sub-plugins.
        app.add_plugins(BidentOfHadesPlugin);
        app.add_plugins(BowOfArtemisPlugin);
    }
}
