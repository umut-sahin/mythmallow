// Disable spawning command prompt on Windows in release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use {
    mythmallow_enemies_sweet::prelude::*,
    mythmallow_game::prelude::*,
    mythmallow_items_greek::prelude::*,
    mythmallow_mode_survival::prelude::*,
    mythmallow_players_greek::prelude::*,
};

fn main() {
    #[cfg(feature = "wasm")]
    {
        // Enable stack traces for panics in WebAssembly.
        console_error_panic_hook::set_once();
    }

    // Create the application with the log plugin.
    let mut app = App::new();
    app.add_plugins(LogPlugin::default());

    // Parse the arguments and initialize the application.
    let args = Args::parse();
    initialize(&mut app, &args);

    // Register and insert arguments.
    app.register_type::<Args>();
    app.insert_resource(args);

    // Add diagnostics plugins.
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(EntityCountDiagnosticsPlugin);

    #[cfg(feature = "development")]
    {
        // Add editor plugin in development mode.
        app.add_plugins(EditorPlugin::default());
    }

    // Add the main plugin.
    app.add_plugins(MythmallowPlugin);

    // Add game mode plugins.
    app.add_plugins(SurvivalModePlugin);
    {
        let game_mode_registry = GAME_MODE_REGISTRY.lock().unwrap();
        let number_of_game_modes = game_mode_registry.len();
        log::info!(
            "{} game mode{} {} registered",
            number_of_game_modes,
            if number_of_game_modes == 1 { "" } else { "s" },
            if number_of_game_modes == 1 { "is" } else { "are" },
        );
    }

    // Add item plugins.
    app.add_plugins(GreekItemsPlugin);
    {
        let item_registry = ITEM_REGISTRY.lock().unwrap();
        let number_of_items = item_registry.len();
        log::info!(
            "{} item{} {} registered",
            number_of_items,
            if number_of_items == 1 { "" } else { "s" },
            if number_of_items == 1 { "is" } else { "are" },
        );
    }

    // Add player plugins.
    app.add_plugins(GreekPlayersPlugin);
    {
        let player_registry = PLAYER_REGISTRY.lock().unwrap();
        let number_of_mythologies = player_registry.len();
        let number_of_players =
            player_registry.iter().map(|(_, players)| players.len()).sum::<usize>();
        log::info!(
            "{} player{} {} registered across {} mytholog{}",
            number_of_players,
            if number_of_players == 1 { "" } else { "s" },
            if number_of_players == 1 { "is" } else { "are" },
            number_of_mythologies,
            if number_of_mythologies == 1 { "y" } else { "ies" },
        );
    }

    // Add enemy plugins.
    app.add_plugins(SweetEnemiesPlugin);
    {
        let enemy_registry = ENEMY_REGISTRY.lock().unwrap();
        let number_of_enemy_packs = enemy_registry.len();
        let number_of_enemies =
            enemy_registry.iter().map(|(_, enemies)| enemies.len()).sum::<usize>();
        log::info!(
            "{} enem{} {} registered across {} enemy pack{}",
            number_of_enemies,
            if number_of_enemies == 1 { "y" } else { "ies" },
            if number_of_enemies == 1 { "is" } else { "are" },
            number_of_enemy_packs,
            if number_of_enemy_packs == 1 { "" } else { "s" },
        );
    }

    // Start the application.
    log::info!("starting the application");
    app.run();
}

#[cfg(feature = "native")]
fn initialize(app: &mut App, args: &Args) {
    // Add default plugins without a window.
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
            })
            .build()
            .disable::<LogPlugin>(),
    );

    // Spawn persistent primary window.
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

    // Add persistent windows plugin.
    app.add_plugins(PersistentWindowsPlugin);

    #[cfg(debug_assertions)]
    {
        // Setup exiting the application with CTRL+Q in development mode.
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
}

#[cfg(feature = "wasm")]
fn initialize(app: &mut App, _args: &Args) {
    // Add default plugins with "fit canvas to parent" of the primary window set.
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window { fit_canvas_to_parent: true, ..default() }),
                ..default()
            })
            .build()
            .disable::<LogPlugin>(),
    );
}
