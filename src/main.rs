// Disable spawning command prompt on Windows outside development mode.
#![cfg_attr(not(feature = "development"), windows_subsystem = "windows")]

use mythmallow::core::{
    dependencies::*,
    plugins::*,
    resources::all::*,
};

fn main() -> AppExit {
    #[cfg(target_family = "wasm")]
    {
        // Enable stack traces for panics in web builds.
        console_error_panic_hook::set_once();
    }

    // Create the application.
    let mut app = App::new();

    // Add log plugin to the application.
    app.add_plugins(LogPlugin::default());

    // Add configuration plugin to the application.
    app.add_plugins(ConfigurationPlugin);

    // Add default plugins to the application.
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Mythmallow".to_owned(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .build()
            .disable::<LogPlugin>(),
    );

    #[cfg(not(target_family = "wasm"))]
    {
        // Find the primary window entity.
        let primary_window_entity = app
            .world_mut()
            .query_filtered::<Entity, With<PrimaryWindow>>()
            .get_single(app.world())
            .expect("fatal: unable to find the primary window");

        // Determine the persistent primary window state path.
        let arguments = app.world().resource::<Arguments>();
        let state_path = arguments.data_directory.join("state").join("window.toml");

        // Make the primary window persistent.
        app.world_mut().entity_mut(primary_window_entity).insert((
            Name::new("Primary Window"),
            PersistentWindowBundle {
                window: Window { title: "Mythmallow".to_owned(), ..Default::default() },
                state: Persistent::<WindowState>::builder()
                    .name("primary window state")
                    .format(StorageFormat::Toml)
                    .path(state_path)
                    .default(WindowState::borderless_fullscreen())
                    .revertible(true)
                    .revert_to_default_on_deserialization_errors(true)
                    .build()
                    .unwrap_or_else(|_| {
                        panic!("fatal: unable to initialize persistent primary window state")
                    }),
            },
        ));

        // Add persistent windows plugin to the application.
        app.add_plugins(PersistentWindowsPlugin);
    }

    // Add diagnostics plugins to the application.
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(EntityCountDiagnosticsPlugin);

    // Start the application.
    log::info!("starting the application");
    app.run()
}
