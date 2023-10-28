use crate::prelude::*;

/// Plugin for managing physics of game objects.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Layer>();

        app.insert_resource(Gravity::ZERO);
        app.insert_resource(PhysicsTimestep::Fixed(1.00 / 180.00));

        app.add_plugins(XpbdPlugin::default());

        app.add_systems(OnExit(GameState::Playing), pause);
        app.add_systems(OnEnter(GameState::Playing), resume);
    }
}
