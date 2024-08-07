use crate::{
    configuration::constants::*,
    prelude::*,
};


/// Database of registered configuration systems.
#[derive(Clone, Copy, Debug, Resource)]
pub struct RegisteredConfigurationSystems {
    pub set_locale: SystemId<LanguageIdentifier>,
}

impl RegisteredConfigurationSystems {
    /// Creates the database.
    pub fn new(app: &mut App, systems: Entity) -> RegisteredConfigurationSystems {
        use super::systems::*;

        let set_locale = app.world_mut().register_system(set_locale);
        RegisteredSystems::attach(app, systems, set_locale, "set_locale");

        RegisteredConfigurationSystems { set_locale }
    }
}


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
    /// Enemies to pick when starting in game.
    pub start_in_game_enemies: Option<String>,
    /// Items to add to the inventory when starting in game.
    pub start_in_game_inventory: Vec<String>,
    /// Level to set when starting in game.
    pub start_in_game_level: Option<NonZeroU16>,
    /// Experience to set when starting in game.
    pub start_in_game_experience: Option<f64>,
    /// Balance to set when starting in game.
    pub start_in_game_balance: Option<f64>,
    /// Number of free refreshes to grant when starting in game.
    pub start_in_game_free_refreshes: Option<usize>,
    /// Flag to enable god mode.
    pub enable_god_mode: bool,
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
    /// ```txt
    /// https://mythmallow.io/?seed=42&game
    /// ```
    pub fn parse() -> Args {
        #[derive(Parser)]
        #[command(about, version)]
        #[clap(name = "mythmallow")]
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
            #[arg(long)]
            pub enemies: Option<String>,
            #[arg(long)]
            pub inventory: Option<String>,
            #[arg(long)]
            pub level: Option<NonZeroU16>,
            #[arg(long)]
            pub experience: Option<f64>,
            #[arg(long)]
            pub balance: Option<f64>,
            #[arg(long)]
            pub free_refreshes: Option<usize>,
            #[arg(long)]
            pub god_mode: bool,
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
                    enemies: None,
                    inventory: None,
                    level: None,
                    experience: None,
                    balance: None,
                    free_refreshes: None,
                    god_mode: false,
                }
            }
        }

        impl Display for ArgsParser {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(configuration) = &self.configuration {
                    write!(f, " --configuration \"{}\"", configuration.display())?;
                }
                if let Some(data) = &self.data {
                    write!(f, " --data \"{}\"", data.display())?;
                }
                if let Some(seed) = &self.seed {
                    write!(f, " --seed {}", seed)?;
                }
                if self.game {
                    write!(f, " --game")?;
                }
                if let Some(mode) = &self.mode {
                    write!(f, " --mode \"{}\"", mode)?;
                }
                if let Some(player) = &self.player {
                    write!(f, " --player \"{}\"", player)?;
                }
                if let Some(enemies) = &self.enemies {
                    write!(f, " --enemies \"{}\"", enemies)?;
                }
                if let Some(inventory) = &self.inventory {
                    write!(f, " --inventory \"{}\"", inventory)?;
                }
                if let Some(level) = &self.level {
                    write!(f, " --level {}", level)?;
                }
                if let Some(experience) = &self.experience {
                    write!(f, " --experience {}", Experience(*experience))?;
                }
                if let Some(balance) = &self.balance {
                    write!(
                        f,
                        " --balance {}",
                        format!("{}", Balance(*balance)).trim_end_matches("$").trim_end(),
                    )?;
                }
                if let Some(free_refreshes) = &self.free_refreshes {
                    write!(f, " --free-refreshes {}", free_refreshes)?;
                }
                if self.god_mode {
                    write!(f, " --god-mode")?;
                }
                Ok(())
            }
        }

        impl ArgsParser {
            pub fn canonicalize(self) -> Args {
                let args = format!("{}", self);
                if !args.is_empty() {
                    log::info!("args:\n\n{}\n", args.trim());
                }

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
                            Path::new("local").join("configuration")
                        }
                    });

                let configuration_directory_display =
                    format!("{}", configuration_directory.display());
                log::info!(
                    "configuration directory:\n\n{}\n",
                    configuration_directory_display
                        .trim_start_matches("\\\\?\\")
                        .trim_start_matches("\"\\\\?\\")
                        .trim_end_matches('"'),
                );

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
                            Path::new("local").join("data")
                        }
                    });

                let data_directory_display = format!("{}", data_directory.display());
                log::info!(
                    "data directory:\n\n{}\n",
                    data_directory_display
                        .trim_start_matches("\\\\?\\")
                        .trim_start_matches("\"\\\\?\\")
                        .trim_end_matches('"'),
                );

                let seed = self.seed;
                let start_in_game = self.game;
                let start_in_game_mode = self.mode;
                let start_in_game_player = self.player;
                let start_in_game_enemies = self.enemies;
                let start_in_game_inventory = self
                    .inventory
                    .unwrap_or_default()
                    .split(',')
                    .map(|item| item.trim().to_owned())
                    .filter(|item| !item.is_empty())
                    .collect();
                let start_in_game_level = self.level;
                let start_in_game_experience = self.experience;
                let start_in_game_balance = self.balance;
                let start_in_game_free_refreshes = self.free_refreshes;
                let enable_god_mode = self.god_mode;

                Args {
                    data_directory,
                    configuration_directory,
                    seed,
                    start_in_game,
                    start_in_game_mode,
                    start_in_game_player,
                    start_in_game_enemies,
                    start_in_game_inventory,
                    start_in_game_level,
                    start_in_game_experience,
                    start_in_game_balance,
                    start_in_game_free_refreshes,
                    enable_god_mode,
                }
            }
        }

        log::info!("version:\n\nv{}\n", env!("CARGO_PKG_VERSION"));

        let help = format!("{}", <ArgsParser as CommandFactory>::command().render_help());
        log::info!("usage:\n\n{}\n", help.trim());

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
                let args = query.replace(['?', '&'], " --");

                let mut parsed_args = vec![];
                let mut parsed_arg = String::new();

                let mut block = false;
                for char in args.trim_start().chars() {
                    match char {
                        '|' => {
                            block = !block;
                        },
                        ' ' | '=' if !block => {
                            parsed_args.push(std::mem::take(&mut parsed_arg));
                        },
                        _ => {
                            parsed_arg.push(char);
                        },
                    }
                }
                if !parsed_arg.is_empty() {
                    parsed_args.push(parsed_arg);
                }

                ArgsParser::try_parse_from(
                    std::iter::once("mythmallow".to_owned()).chain(parsed_args),
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
    pub locale: String,

    pub pause_on_losing_focus: bool,
    pub show_diagnostics_overlay: bool,

    #[cfg(feature = "development")]
    pub enable_physics_gizmos: bool,
}

impl GeneralSettings {
    /// Initializes the resource in the app.
    pub fn initialize(app: &mut App) {
        let args = app.world().resource::<Args>();
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

impl GeneralSettings {
    /// Gets the locale specified in the general settings of the game.
    pub fn locale(&self) -> Option<LanguageIdentifier> {
        self.locale.parse::<LanguageIdentifier>().ok()
    }
}

impl Default for GeneralSettings {
    fn default() -> GeneralSettings {
        GeneralSettings {
            locale: DefaultLocale::get(&SupportedLocales::get()).identifier().to_string(),

            pause_on_losing_focus: true,
            show_diagnostics_overlay: false,

            #[cfg(feature = "development")]
            enable_physics_gizmos: false,
        }
    }
}


/// Supported locales of the game.
#[derive(Clone, Default, Deref, Resource)]
pub struct SupportedLocales(pub Vec<LanguageIdentifier>);

impl SupportedLocales {
    /// Gets the supported locales.
    pub fn get() -> SupportedLocales {
        SupportedLocales(
            ["en-US", "tr"]
                .iter()
                .map(|s| s.parse::<LanguageIdentifier>().expect("invalid locale constant"))
                .collect(),
        )
    }
}


/// Default locale of the game in the current platform.
#[derive(Clone, Deref, Resource)]
pub struct DefaultLocale(pub LanguageIdentifier);

impl DefaultLocale {
    /// Gets the default locale.
    pub fn get(supported_locales: &SupportedLocales) -> DefaultLocale {
        let default_locale = sys_locale::get_locale()
            .unwrap_or_else(|| "en-US".to_owned())
            .parse::<LanguageIdentifier>()
            .map(DefaultLocale);

        if let Ok(default_locale) = default_locale {
            if supported_locales.contains(&default_locale.0) {
                return default_locale;
            }
        }

        DefaultLocale("en-US".parse::<LanguageIdentifier>().unwrap())
    }
}

impl DefaultLocale {
    /// Gets the identifier of the default locale.
    pub fn identifier(&self) -> LanguageIdentifier {
        self.0.clone()
    }
}


/// Asset locations of locale assets.
#[derive(Debug, Default, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct LocaleAssets(pub Vec<&'static str>);


/// Asset handles of locale resources.
#[derive(Clone, Default, Deref, Resource)]
pub struct LocaleResourceHandles(pub Vec<Handle<ResourceAsset>>);


/// Locales folder.
#[derive(Clone, Default, Deref, Resource)]
pub struct LocalesFolder(pub Handle<LoadedFolder>);


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
    /// Keys to open the market, when the game mode allows it.
    pub market: SmallVec<[KeyCode; 1]>,
}

impl KeyBindings {
    /// Initializes the resource in the app.
    pub fn initialize(app: &mut App) {
        let args = app.world().resource::<Args>();
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
            up: smallvec![KeyCode::KeyW, KeyCode::ArrowUp],
            left: smallvec![KeyCode::KeyA, KeyCode::ArrowLeft],
            down: smallvec![KeyCode::KeyS, KeyCode::ArrowDown],
            right: smallvec![KeyCode::KeyD, KeyCode::ArrowRight],
            dash: smallvec![KeyCode::Space],
            pause: smallvec![KeyCode::Escape],
            market: smallvec![KeyCode::KeyB],
        }
    }
}
