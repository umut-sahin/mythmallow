use crate::{
    diagnostics::systems::*,
    prelude::*,
};
use crate::diagnostics::components::*;

/// Plugin for the properties of game objects.
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Diagnostic>();

        app.add_systems(OnEnter(DiagnosticsState::FPS), text_setup);
        app.add_systems(Update, update_fps_text);
        app.add_systems(OnExit(DiagnosticsState::FPS), remove_fps_text);
    }
}