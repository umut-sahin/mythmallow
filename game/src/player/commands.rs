use crate::prelude::*;

/// Controls the player.
#[derive(ConsoleCommand, Parser)]
#[command(name = "player")]
#[command(disable_help_flag = true)]
pub struct PlayerCommand {
    #[clap(subcommand)]
    pub subcommand: PlayerCommands,
}

/// Player commands.
#[derive(Debug, Subcommand)]
pub enum PlayerCommands {
    /// Controls the god mode.
    GodMode {
        #[clap(subcommand)]
        subcommand: GodModeCommands,
    },
}

/// God mode commands.
#[derive(Debug, Subcommand)]
pub enum GodModeCommands {
    /// Shows the status of god mode.
    Status,
    /// Enables god mode.
    Enable,
    /// Disables god mode.
    Disable,
}
