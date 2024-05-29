use crate::{
    camera::plugin::CameraPlugin,
    combat::plugin::CombatPlugin,
    configuration::plugin::ConfigurationPlugin,
    console::plugin::ConsolePlugin,
    core::plugin::CorePlugin,
    enemy::plugin::EnemyPlugin,
    input::plugin::InputPlugin,
    inventory::plugin::InventoryPlugin,
    item::plugin::ItemPlugin,
    leveling::plugin::LevelingPlugin,
    map::plugin::MapPlugin,
    market::plugin::MarketPlugin,
    mode::plugin::ModePlugin,
    movement::plugin::MovementPlugin,
    physics::plugin::PhysicsPlugin,
    player::plugin::PlayerPlugin,
    prelude::*,
    property::plugin::PropertyPlugin,
    status_effect::plugin::StatusEffectPlugin,
    ui::plugin::UiPlugin,
};

/// Main plugin.
pub struct MythmallowPlugin;

impl Plugin for MythmallowPlugin {
    fn build(&self, app: &mut App) {
        // Add sub-plugins.
        app.add_plugins(ConfigurationPlugin);
        app.add_plugins(CorePlugin);
        app.add_plugins(InputPlugin);
        app.add_plugins(ConsolePlugin);
        app.add_plugins(CameraPlugin);
        app.add_plugins(UiPlugin);
        app.add_plugins(PhysicsPlugin);
        app.add_plugins(ModePlugin);
        app.add_plugins(ItemPlugin);
        app.add_plugins(MarketPlugin);
        app.add_plugins(InventoryPlugin);
        app.add_plugins(MapPlugin);
        app.add_plugins(PropertyPlugin);
        app.add_plugins(LevelingPlugin);
        app.add_plugins(StatusEffectPlugin);
        app.add_plugins(MovementPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(EnemyPlugin);
        app.add_plugins(CombatPlugin);
    }
}
