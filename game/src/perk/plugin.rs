use crate::{
    perk::{
        commands::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing the perks.
pub struct PerkPlugin;

impl Plugin for PerkPlugin {
    fn build(&self, app: &mut App) {
        // Initialize registry.
        app.init_resource::<PerkRegistry>();

        // Add events.
        app.add_event::<PerkObtainedEvent>();
        app.add_event::<PerkLostEvent>();

        // Add console commands.
        app.add_console_command::<PerkCommand, _>(apply_perk_command);
    }
}
