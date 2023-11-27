use crate::{
    configuration::constants::*,
    prelude::*,
};


/// Resource for the arguments of the application.
#[derive(Debug, Reflect, Resource)]
pub struct Args {
    /// Base path for configuration files.
    pub configuration_directory: PathBuf,
    /// Base path for data files.
    pub data_directory: PathBuf,
    /// Seed for random number generation.
    pub seed: Option<u64>,
    /// Flag for starting the application in game.
    pub start_in_game: bool,
    /// Game mode to pick when starting in game.
    pub start_in_game_mode: Option<String>,
    /// Player to pick when starting in game.
    pub start_in_game_player: Option<String>,
}

impl Args {
    /// Parses the arguments from the environment.
    ///
    /// # Native
    ///
    /// Arguments are parsed from the command line arguments.
    ///
    /// ```shell
    /// mythmallow --seed 42 --game
    /// ```
    ///
    /// # WebAssembly
    ///
    /// Arguments are parsed from the URL.
    ///
    /// ```shell
    /// https://mythmallow.io/?seed=42&game
    /// ```
    pub fn parse() -> Args {
        #[derive(Parser)]
        #[command(about, version)]
        struct ArgsParser {
            #[arg(long)]
            pub configuration: Option<PathBuf>,
            #[arg(long)]
            pub data: Option<PathBuf>,
            #[arg(long)]
            pub seed: Option<u64>,
            #[arg(long)]
            pub game: bool,
            #[arg(long)]
            pub mode: Option<String>,
            #[arg(long)]
            pub player: Option<String>,
        }

        impl Default for ArgsParser {
            fn default() -> ArgsParser {
                ArgsParser {
                    configuration: None,
                    data: None,
                    seed: None,
                    game: false,
                    mode: None,
                    player: None,
                }
            }
        }

        impl Display for ArgsParser {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(data) = &self.data {
                    write!(f, " --data {:?}", data)?;
                }
                if let Some(configuration) = &self.configuration {
                    write!(f, " --configuration {:?}", configuration)?;
                }
                if let Some(seed) = &self.seed {
                    write!(f, " --seed {}", seed)?;
                }
                if self.game {
                    write!(f, " --game")?;
                }
                if let Some(mode) = &self.mode {
                    write!(f, " --mode {:?}", mode)?;
                }
                if let Some(player) = &self.player {
                    write!(f, " --player {:?}", player)?;
                }
                Ok(())
            }
        }

        impl ArgsParser {
            pub fn canonicalize(self) -> Args {
                log::info!("version: v{}", env!("CARGO_PKG_VERSION"));
                log::info!("args:{}", self);

                let configuration_directory = self
                    .configuration
                    .as_ref()
                    .map(|path| path.canonicalize().unwrap_or(path.to_owned()))
                    .unwrap_or_else(|| {
                        #[cfg(feature = "native")]
                        {
                            dirs::config_dir()
                                .map(|platform_config_dir| platform_config_dir.join("mythmallow"))
                                .unwrap_or_else(|| {
                                    panic!(
                                        "fatal: unable to determine the configuration directory",
                                    );
                                })
                        }
                        #[cfg(feature = "wasm")]
                        {
                            Path::new("session").join("configuration")
                        }
                    });

                log::info!("configuration directory: {:?}", configuration_directory);

                let data_directory = self
                    .data
                    .as_ref()
                    .map(|path| path.canonicalize().unwrap_or(path.to_owned()))
                    .unwrap_or_else(|| {
                        #[cfg(feature = "native")]
                        {
                            dirs::data_dir()
                                .map(|platform_data_dir| platform_data_dir.join("mythmallow"))
                                .unwrap_or_else(|| {
                                    panic!("fatal: unable to determine the data directory");
                                })
                        }
                        #[cfg(feature = "wasm")]
                        {
                            Path::new("session").join("data")
                        }
                    });

                log::info!("data directory: {:?}", data_directory);

                let seed = self.seed;
                let start_in_game = self.game;
                let start_in_game_mode = self.mode;
                let start_in_game_player = self.player;

                Args {
                    data_directory,
                    configuration_directory,
                    seed,
                    start_in_game,
                    start_in_game_mode,
                    start_in_game_player,
                }
            }
        }

        #[cfg(feature = "native")]
        {
            ArgsParser::parse().canonicalize()
        }
        #[cfg(feature = "wasm")]
        {
            let query = web_sys::window()
                .and_then(|window| window.location().search().ok())
                .unwrap_or("".to_owned());

            if query.is_empty() {
                ArgsParser::default().canonicalize()
            } else {
                ArgsParser::try_parse_from(
                    std::iter::once("mythmallow".to_owned()).chain(
                        query
                            .trim_start_matches('?')
                            .split('&')
                            .flat_map(|option| {
                                let mut option = option.split('=');

                                let key = format!("--{}", option.next().unwrap());
                                let value = option.fold(String::new(), std::ops::Add::add);

                                [key, value]
                            })
                            .filter(|arg| !arg.is_empty()),
                    ),
                )
                .unwrap_or_else(|error| {
                    let full_error = format!("{}", error);
                    let short_error = full_error.split('\n').next().unwrap();

                    let error = short_error.trim_start_matches("error: ");
                    log::error!("unable to parse the arguments from the URL ({})", error);

                    ArgsParser::default()
                })
                .canonicalize()
            }
        }
    }
}


/// Resource for the general settings of the application.
#[derive(Debug, Deserialize, Reflect, Resource, Serialize)]
#[serde(default)]
pub struct GeneralSettings {
    pub pause_on_losing_focus: bool,
    pub show_diagnostics_overlay: bool,
}

impl GeneralSettings {
    /// Initializes the resource in the app.
    pub fn initialize(app: &mut App) {
        let args = app.world.resource::<Args>();
        app.insert_resource(
            Persistent::<GeneralSettings>::builder()
                .name("general settings")
                .format(CONFIGURATION_STORAGE_FORMAT)
                .path({
                    #[cfg(feature = "native")]
                    {
                        args.configuration_directory.join("general-settings.toml")
                    }
                    #[cfg(feature = "wasm")]
                    {
                        args.configuration_directory.join("general-settings")
                    }
                })
                .default(GeneralSettings::default())
                .revertible(true)
                .build()
                .unwrap_or_else(|_| panic!("fatal: unable to initialize the general settings")),
        );
    }
}

impl Default for GeneralSettings {
    fn default() -> GeneralSettings {
        GeneralSettings { pause_on_losing_focus: true, show_diagnostics_overlay: false }
    }
}


/// Resource for the key bindings of the application.
#[derive(Debug, Deserialize, Reflect, Resource, Serialize)]
#[serde(default)]
pub struct KeyBindings {
    /// Keys to go up.
    pub up: SmallVec<[KeyCode; 2]>,
    /// Keys to go left.
    pub left: SmallVec<[KeyCode; 2]>,
    /// Keys to go down.
    pub down: SmallVec<[KeyCode; 2]>,
    /// Keys to go right.
    pub right: SmallVec<[KeyCode; 2]>,
    /// Keys to dash.
    pub dash: SmallVec<[KeyCode; 1]>,
    /// Keys to pause the game.
    pub pause: SmallVec<[KeyCode; 1]>,
}

impl KeyBindings {
    /// Initializes the resource in the app.
    pub fn initialize(app: &mut App) {
        let args = app.world.resource::<Args>();
        app.insert_resource(
            Persistent::<KeyBindings>::builder()
                .name("key bindings")
                .format(CONFIGURATION_STORAGE_FORMAT)
                .path({
                    #[cfg(feature = "native")]
                    {
                        args.configuration_directory.join("key-bindings.toml")
                    }
                    #[cfg(feature = "wasm")]
                    {
                        args.configuration_directory.join("key-bindings")
                    }
                })
                .default(KeyBindings::default())
                .revertible(true)
                .build()
                .unwrap_or_else(|_| panic!("fatal: unable to initialize the key bindings")),
        );
    }
}

impl Default for KeyBindings {
    fn default() -> KeyBindings {
        KeyBindings {
            up: smallvec![KeyCode::W, KeyCode::Up],
            left: smallvec![KeyCode::A, KeyCode::Left],
            down: smallvec![KeyCode::S, KeyCode::Down],
            right: smallvec![KeyCode::D, KeyCode::Right],
            dash: smallvec![KeyCode::Space],
            pause: smallvec![KeyCode::Escape],
        }
    }
}
