//! Configuration resources.

use mythmallow_core_dependencies::*;


/// Resource for the arguments of the application.
#[derive(Debug, Reflect, Resource)]
pub struct Arguments {
    /// Base directory for configuration files.
    pub configuration_directory: PathBuf,
    /// Base directory for data files.
    pub data_directory: PathBuf,
}

impl Arguments {
    /// Initializes the arguments from the environment.
    ///
    /// # Native
    ///
    /// Arguments are parsed from the command line.
    ///
    /// ```shell
    /// mythmallow --seed 42 --game
    /// ```
    ///
    /// # Web
    ///
    /// Arguments are parsed from the URL.
    ///
    /// ```txt
    /// https://mythmallow.io/?seed=42&game
    /// ```
    pub fn initialize(app: &mut App) {
        #[derive(Parser)]
        #[command(about, version)]
        #[clap(name = "mythmallow")]
        struct ArgumentParser {
            /// Set base directory for configuration files.
            #[arg(long)]
            pub configuration: Option<PathBuf>,

            /// Set base directory for data files.
            #[arg(long)]
            pub data: Option<PathBuf>,
        }

        impl Default for ArgumentParser {
            fn default() -> ArgumentParser {
                ArgumentParser { configuration: None, data: None }
            }
        }

        impl Display for ArgumentParser {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(configuration) = &self.configuration {
                    write!(f, " --configuration \"{}\"", configuration.display())?;
                }
                if let Some(data) = &self.data {
                    write!(f, " --data \"{}\"", data.display())?;
                }
                Ok(())
            }
        }

        log::info!("version:\n\nv{}\n", env!("CARGO_PKG_VERSION"));

        let help = format!("{}", <ArgumentParser as CommandFactory>::command().render_help());
        log::info!("usage:\n\n{}\n", help.trim().trim_start_matches("Usage: "));

        let parser = {
            #[cfg(not(target_family = "wasm"))]
            {
                ArgumentParser::parse()
            }
            #[cfg(target_family = "wasm")]
            {
                let query = web_sys::window()
                    .and_then(|window| window.location().search().ok())
                    .unwrap_or("".to_owned());

                if query.is_empty() {
                    ArgumentParser::default()
                } else {
                    let processed_query = query.replace(['?', '&'], " --");

                    let mut parsed_arguments = vec![];
                    let mut parsed_argument = String::new();

                    let mut block = false;
                    for char in processed_query.trim_start().chars() {
                        match char {
                            '|' => {
                                block = !block;
                            },
                            ' ' | '=' if !block => {
                                parsed_arguments.push(std::mem::take(&mut parsed_argument));
                            },
                            _ => {
                                parsed_argument.push(char);
                            },
                        }
                    }
                    if !parsed_argument.is_empty() {
                        parsed_arguments.push(parsed_argument);
                    }

                    ArgumentParser::try_parse_from(
                        std::iter::once("mythmallow".to_owned()).chain(parsed_arguments),
                    )
                    .unwrap_or_else(|error| {
                        let full_error = format!("{}", error);
                        let short_error = full_error.split('\n').next().unwrap();

                        let error = short_error.trim_start_matches("error: ");
                        log::error!(
                            "unable to parse the arguments from the URL ({})",
                            error.replace("--", ""),
                        );

                        ArgumentParser::default()
                    })
                }
            }
        };

        let parser_display = format!("{}", parser);
        if !parser_display.is_empty() {
            log::info!("arguments:\n\n{}\n", parser_display.trim());
        }

        let configuration_directory = parser
            .configuration
            .as_ref()
            .map(|path| path.canonicalize().unwrap_or(path.to_owned()))
            .unwrap_or_else(|| {
                #[cfg(not(target_family = "wasm"))]
                {
                    dirs::config_dir()
                        .map(|platform_config_dir| platform_config_dir.join("mythmallow"))
                        .unwrap_or_else(|| {
                            panic!("fatal: unable to determine the configuration directory",);
                        })
                }
                #[cfg(target_family = "wasm")]
                {
                    Path::new("local").join("configuration")
                }
            });

        let configuration_directory_display = format!("{}", configuration_directory.display());
        log::info!(
            "configuration directory:\n\n{}\n",
            configuration_directory_display
                .trim_start_matches("\\\\?\\")
                .trim_start_matches("\"\\\\?\\")
                .trim_end_matches('"'),
        );

        let data_directory = parser
            .data
            .as_ref()
            .map(|path| path.canonicalize().unwrap_or(path.to_owned()))
            .unwrap_or_else(|| {
                #[cfg(not(target_family = "wasm"))]
                {
                    dirs::data_dir()
                        .map(|platform_data_dir| platform_data_dir.join("mythmallow"))
                        .unwrap_or_else(|| {
                            panic!("fatal: unable to determine the data directory");
                        })
                }
                #[cfg(target_family = "wasm")]
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

        let arguments = Arguments { data_directory, configuration_directory };
        app.insert_resource(arguments);
    }
}
