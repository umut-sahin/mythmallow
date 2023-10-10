use crate::{
    configuration::constants::*,
    prelude::*,
};


/// Configuration resource for arguments of the application.
#[derive(Debug, Reflect, Resource)]
pub struct Args {
    /// Base path for configuration files.
    pub configuration_directory: PathBuf,
    /// Base path for data files.
    pub data_directory: PathBuf,
    /// Flag to start the application in the game.
    pub start_in_game: bool,
}

impl Args {
    /// Parse the arguments from the environment.
    ///
    /// # Native
    ///
    /// Arguments are parsed from the command line arguments.
    ///
    /// ```shell
    /// mythmellow --game --data custom/path/to/data
    /// ```
    ///
    /// # WebAssembly
    ///
    /// Arguments are parsed from the URL.
    ///
    /// ```shell
    /// https://mythmellow.io/play?game&data=session/custom/path/to/data
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
            pub game: bool,
        }

        impl Default for ArgsParser {
            fn default() -> ArgsParser {
                ArgsParser { configuration: None, data: None, game: false }
            }
        }

        impl ArgsParser {
            pub fn canonicalize(self) -> Args {
                let configuration_directory = self
                    .configuration
                    .as_ref()
                    .map(|path| path.canonicalize().unwrap_or(path.to_owned()))
                    .unwrap_or_else(|| {
                        #[cfg(feature = "native")]
                        {
                            dirs::config_dir()
                                .map(|platform_config_dir| platform_config_dir.join("mythmellow"))
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

                let data_directory = self
                    .data
                    .as_ref()
                    .map(|path| path.canonicalize().unwrap_or(path.to_owned()))
                    .unwrap_or_else(|| {
                        #[cfg(feature = "native")]
                        {
                            dirs::data_dir()
                                .map(|platform_data_dir| platform_data_dir.join("mythmellow"))
                                .unwrap_or_else(|| {
                                    panic!("fatal: unable to determine the data directory");
                                })
                        }
                        #[cfg(feature = "wasm")]
                        {
                            Path::new("session").join("data")
                        }
                    });

                let start_in_game = self.game;

                Args { configuration_directory, data_directory, start_in_game }
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
                    std::iter::once("mythmellow".to_owned()).chain(
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
                    // error starts with the string "error: " and we don't want that
                    let error = &short_error[7..];
                    panic!("fatal: unable to parse the arguments ({})", error);
                })
                .canonicalize()
            }
        }
    }
}


/// Configuration resource for key bindings of the application.
#[derive(Debug, Deserialize, Reflect, Resource, Serialize)]
#[serde(default)]
pub struct KeyBindings {
    pub up: SmallVec<[KeyCode; 2]>,
    pub left: SmallVec<[KeyCode; 2]>,
    pub down: SmallVec<[KeyCode; 2]>,
    pub right: SmallVec<[KeyCode; 2]>,
    pub dash: SmallVec<[KeyCode; 1]>,
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
                .unwrap_or_else(|_| panic!("fatal: unable to initialize persistent key bindings")),
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
