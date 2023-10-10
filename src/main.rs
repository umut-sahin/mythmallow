// Disable spawning command prompt in windows for release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use {
    bevy::prelude::*,
    mythmellow::prelude::*,
};

fn main() {
    #[cfg(feature = "wasm")]
    {
        // Enable stack traces for panics in WebAssembly.
        console_error_panic_hook::set_once();
    }

    let args = Args::parse();

    let mut app = initialize(&args);
    app.register_type::<Args>().insert_resource(args);

    app.add_plugins(MythmellowPlugin);

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::{
            EntityCountDiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
        };
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(EntityCountDiagnosticsPlugin);

        #[cfg(feature = "bevy_editor_pls")]
        {
            use bevy_editor_pls::EditorPlugin;
            app.add_plugins(EditorPlugin::default());
        }
    }

    app.run();
}

#[cfg(feature = "native")]
fn initialize(args: &Args) -> App {
    use {
        bevy::window::{
            ExitCondition,
            PrimaryWindow,
        },
        bevy_persistent::prelude::*,
        bevy_persistent_windows::prelude::*,
    };

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
            })
            .build(),
    );

    app.world.spawn((
        Name::new("Primary Window"),
        PrimaryWindow,
        PersistentWindowBundle {
            window: Window { title: "Mythmellow".to_owned(), ..Default::default() },
            state: Persistent::<WindowState>::builder()
                .name("window state")
                .format(StorageFormat::Toml)
                .path(args.data_directory.join("state").join("window.toml"))
                .default(WindowState::borderless_fullscreen())
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .unwrap_or_else(|_| panic!("fatal: unable to initialize persistent window state")),
        },
    ));

    app.add_plugins(PersistentWindowsPlugin);

    #[cfg(debug_assertions)]
    {
        use bevy::app::AppExit;

        fn exit_with_ctrl_q(
            keyboard_input: Res<Input<KeyCode>>,
            mut app_exit_events: ResMut<Events<AppExit>>,
        ) {
            if keyboard_input.pressed(KeyCode::ControlLeft)
                && keyboard_input.just_pressed(KeyCode::Q)
            {
                app_exit_events.send(AppExit);
            }
        }

        app.add_systems(Update, exit_with_ctrl_q);
    }

    app
}

#[cfg(feature = "wasm")]
fn initialize(_args: &Args) -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app
}
