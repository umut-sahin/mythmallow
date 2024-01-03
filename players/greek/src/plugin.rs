use {
    crate::{
        artemis::ArtemisPlugin,
        hades::HadesPlugin,
    },
    mythmallow::prelude::*,
};

/// Plugin for managing the players from "Greek" mythology.
pub struct GreekPlayersPlugin;

impl Plugin for GreekPlayersPlugin {
    fn build(&self, app: &mut App) {
        // Add sub-plugins.
        app.add_plugins(ArtemisPlugin);
        app.add_plugins(HadesPlugin);
    }
}
