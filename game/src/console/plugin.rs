use crate::{
    console::systems::*,
    prelude::*,
};

/// Plugin for managing the console.
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        // Insert resources.
        app.insert_resource(ConsoleConfiguration::default());

        // Add sub-plugins.
        app.add_plugins(BevyConsolePlugin);

        // Set console position.
        {
            let primary_window =
                app.world.query_filtered::<&Window, With<PrimaryWindow>>().single(&app.world);

            let window_width = primary_window.width();
            let left_margin = window_width * 0.1;
            let console_width = window_width * 0.8;

            let window_height = primary_window.height();
            let top_margin = window_height * 0.1;
            let console_height = window_height * 0.8;

            app.insert_resource(ConsoleConfiguration {
                left_pos: left_margin,
                width: console_width,
                top_pos: top_margin,
                height: console_height,
                symbol: "\n> ".to_owned(),
                ..default()
            });
        }

        // Add systems.
        app.add_systems(
            Update,
            control_physics_time
                .run_if(|console_state: Res<ConsoleState>| console_state.is_changed()),
        );
    }
}
