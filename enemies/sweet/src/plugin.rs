use crate::{
    chocolate_bar::ChocolateBarPlugin,
    gummy_bear::GummyBearPlugin,
    prelude::*,
};

/// Plugin for managing the enemies from "Sweet" enemy pack.
pub struct SweetEnemiesPlugin;

impl Plugin for SweetEnemiesPlugin {
    fn build(&self, app: &mut App) {
        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("content/enemies/sweet.ftl");

        // Add sub-plugins.
        app.add_plugins(ChocolateBarPlugin);
        app.add_plugins(GummyBearPlugin);
    }
}
