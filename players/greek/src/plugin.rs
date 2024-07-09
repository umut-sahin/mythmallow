use crate::{
    artemis::ArtemisPlugin,
    hades::HadesPlugin,
    prelude::*,
};

/// Plugin for managing the players from "Greek" mythology.
pub struct GreekPlayersPlugin;

impl Plugin for GreekPlayersPlugin {
    fn build(&self, app: &mut App) {
        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("content/players/greek.ftl");

        // Add sub-plugins.
        app.add_plugins(ArtemisPlugin);
        app.add_plugins(HadesPlugin);
    }
}
