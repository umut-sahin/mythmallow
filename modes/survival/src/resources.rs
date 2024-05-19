use {
    crate::constants::*,
    mythmallow::prelude::*,
};


/// Resource for the arguments of the "Survival" mode.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct SurvivalModeArgs {
    /// Wave to start when starting in game.
    pub start_in_game_waves: Option<NonZeroU8>,
}

impl SurvivalModeArgs {
    /// Parses the arguments of the "Survival" mode from the environment.
    ///
    /// # Native
    ///
    /// Arguments are parsed from the "mode" command line argument.
    ///
    /// ```shell
    /// mythmallow --game --mode "survival --wave 3"
    /// ```
    ///
    /// # WebAssembly
    ///
    /// Arguments are parsed from the "mode" query parameter.
    ///
    /// ```txt
    /// https://mythmallow.io/?game&mode=|survival?wave=2|
    /// ```
    pub fn parse<'i>(args: impl Iterator<Item = &'i str>) -> Result<SurvivalModeArgs, clap::Error> {
        #[derive(Parser)]
        #[clap(name = "survival")]
        #[clap(disable_help_flag = true)]
        #[clap(disable_help_subcommand = true)]
        struct ArgsParser {
            #[arg(long)]
            pub wave: Option<NonZeroU8>,
        }

        impl Default for ArgsParser {
            fn default() -> ArgsParser {
                ArgsParser { wave: None }
            }
        }

        impl Display for ArgsParser {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(wave) = &self.wave {
                    write!(f, " --wave {}", wave)?;
                }
                Ok(())
            }
        }

        impl ArgsParser {
            pub fn canonicalize(self) -> SurvivalModeArgs {
                let survival_mode_args = format!("{}", self);
                if !survival_mode_args.is_empty() {
                    log::info!("survival mode args:\n\n{}\n", survival_mode_args.trim());
                }

                let start_in_game_waves = self.wave;

                SurvivalModeArgs { start_in_game_waves }
            }
        }

        log::info!("survival mode version:\n\nv{}\n", env!("CARGO_PKG_VERSION"));

        let help = format!("{}", <ArgsParser as CommandFactory>::command().render_help());
        log::info!("survival mode usage:\n\n{}\n", help.trim());

        ArgsParser::try_parse_from(args).map(|args| args.canonicalize()).map_err(|error| {
            log::error!("failed to parse the arguments of the survival mode\n\n{}", error);
            error
        })
    }
}


/// Resource for the current wave.
#[derive(Clone, Copy, Debug, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct CurrentWave(pub NonZeroU8);

impl CurrentWave {
    // Gets the index of the current wave.
    pub fn index(&self) -> usize {
        (self.get() - 1) as usize
    }

    // Gets the current wave is the last wave.
    pub fn is_last(&self) -> bool {
        self.get() >= WAVES
    }
}

impl CurrentWave {
    // Increments the current wave.
    pub fn increment(&mut self) {
        if self.is_last() {
            panic!("tried to go past the last wave");
        }
        self.0 = self.checked_add(1).unwrap();
    }
}

impl Default for CurrentWave {
    fn default() -> CurrentWave {
        CurrentWave(NonZeroU8::new(1).unwrap())
    }
}


/// Resource for the remaining time to complete the current wave.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct WaveTimer(pub Timer);

impl WaveTimer {
    /// Creates a new wave timer.
    pub fn new(duration: Duration) -> WaveTimer {
        WaveTimer(Timer::new(duration, TimerMode::Once))
    }
}

impl Default for WaveTimer {
    fn default() -> WaveTimer {
        WaveTimer(Timer::new(Duration::from_secs(60), TimerMode::Once))
    }
}


/// Resource for the duration of waves.
#[derive(Debug, Default, Deref, Reflect, Resource)]
#[reflect(Resource)]
pub struct WaveDurations(Vec<Duration>);

impl WaveDurations {
    /// Creates wave durations.
    pub fn new(waves: u8) -> WaveDurations {
        let mut result = Vec::with_capacity(waves as usize);
        for wave in 1..=waves {
            match wave {
                1 => {
                    result.push(Duration::from_secs_f32(10.00));
                },
                2 => {
                    result.push(Duration::from_secs_f32(15.00));
                },
                3 => {
                    result.push(Duration::from_secs_f32(20.00));
                },
                _ => {
                    result.push(Duration::from_secs_f32(30.00));
                },
            }
        }
        WaveDurations(result)
    }
}

impl Index<CurrentWave> for WaveDurations {
    type Output = Duration;

    fn index(&self, current_wave: CurrentWave) -> &Duration {
        &self.0[current_wave.index()]
    }
}
