use crate::prelude::*;

/// Controls the perks.
#[derive(ConsoleCommand, Parser)]
#[command(name = "perk")]
#[command(disable_help_flag = true)]
pub struct PerkCommand {
    #[clap(subcommand)]
    pub subcommand: PerkCommands,
}

/// Perk commands.
#[derive(Debug, Subcommand)]
pub enum PerkCommands {
    /// Lists the available perks.
    List,
    /// Obtains a perk.
    Obtain { perk: SmolStr },
    /// Loses a perk.
    Lose { perk: SmolStr },
    /// Controls the commonness of perks.
    Commonness {
        #[clap(subcommand)]
        subcommand: PerkCommonnessCommands,
    },
}

/// Perk commonness commands.
#[derive(Debug, Subcommand)]
pub enum PerkCommonnessCommands {
    /// Shows the commonness of a perk.
    #[clap(arg_required_else_help = true)]
    Show { perk: SmolStr },
    /// Sets the commonness of a perk.
    #[clap(arg_required_else_help = true)]
    Set { perk: SmolStr, commonness: u64 },
}
