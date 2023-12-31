use crate::{
    map::systems::*,
    prelude::*,
};

/// Plugin for managing the map.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<MapBounds>();

        // Register components.
        app.register_type::<Map>();
        app.register_type::<MapBound>();

        // Add systems.
        app.add_systems(OnEnter(GameState::Won), despawn_map);
        app.add_systems(OnEnter(GameState::Over), despawn_map);
        app.add_systems(OnEnter(GameState::Restart), despawn_map.in_set(RestartSystems::Map));
        app.add_systems(OnExit(AppState::Game), despawn_map);
    }
}
