use crate::{
    camera::plugin::CameraPlugin,
    configuration::plugin::ConfigurationPlugin,
    core::plugin::CorePlugin,
    enemy::plugin::EnemyPlugin,
    input::plugin::InputPlugin,
    map::plugin::MapPlugin,
    movement::plugin::MovementPlugin,
    physics::plugin::PhysicsPlugin,
    player::plugin::PlayerPlugin,
    prelude::*,
    property::plugin::PropertyPlugin,
    status_effect::plugin::StatusEffectPlugin,
    ui::plugin::UiPlugin,
};

/// Main plugin.
pub struct MythmellowPlugin;

impl Plugin for MythmellowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CorePlugin);
        app.add_plugins(ConfigurationPlugin);
        app.add_plugins(InputPlugin);
        app.add_plugins(CameraPlugin);
        app.add_plugins(UiPlugin);
        app.add_plugins(PhysicsPlugin);
        app.add_plugins(MapPlugin);
        app.add_plugins(PropertyPlugin);
        app.add_plugins(StatusEffectPlugin);
        app.add_plugins(MovementPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(EnemyPlugin);
    }
}
