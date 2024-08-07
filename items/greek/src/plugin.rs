use crate::{
    bident_of_hades::BidentOfHadesPlugin,
    bow_of_artemis::BowOfArtemisPlugin,
    prelude::*,
};

/// Plugin for managing the items from "Greek" mythology.
pub struct GreekItemsPlugin;

impl Plugin for GreekItemsPlugin {
    fn build(&self, app: &mut App) {
        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("content/items/greek.ftl");

        // Add sub-plugins.
        app.add_plugins(BidentOfHadesPlugin);
        app.add_plugins(BowOfArtemisPlugin);
    }
}
