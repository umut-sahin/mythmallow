use crate::prelude::*;


/// Controls the experience of the player.
#[derive(ConsoleCommand, Parser)]
#[command(name = "experience")]
#[command(disable_help_flag = true)]
pub struct ExperienceCommand {
    #[clap(subcommand)]
    pub subcommand: ExperienceCommands,
}

/// Experience commands.
#[derive(Debug, Subcommand)]
pub enum ExperienceCommands {
    /// Shows the experience of the player.
    Show,
    /// Increases the experience of the player.
    Add { experience: f64 },
}


/// Controls the level of the player.
#[derive(ConsoleCommand, Parser)]
#[command(name = "level")]
#[command(disable_help_flag = true)]
pub struct LevelCommand {
    #[clap(subcommand)]
    pub subcommand: LevelCommands,
}

/// Level commands.
#[derive(Debug, Subcommand)]
pub enum LevelCommands {
    /// Shows the level of the player.
    Show,
    /// Sets the level of the player.
    Set { level: NonZeroU16 },
}
