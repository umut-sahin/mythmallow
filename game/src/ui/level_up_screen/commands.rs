use crate::prelude::*;

/// Controls the level up screen.
#[derive(ConsoleCommand, Parser)]
#[command(name = "level-up-screen")]
#[command(disable_help_flag = true)]
pub struct LevelUpScreenCommand {
    #[clap(subcommand)]
    pub subcommand: LevelUpScreenCommands,
}

/// Level up screen commands.
#[derive(Debug, Subcommand)]
pub enum LevelUpScreenCommands {
    /// Opens the level up screen.
    Open,
    /// Closes the level up screen.
    Close,
    /// Show the perks offered in the level up screen.
    Show,
    /// Selects a perk offered in the level up screen.
    Select { position: NonZeroUsize },
    /// Rerolls the perks offered in the level up screen.
    Reroll,
    /// Offers a perk in the level up screen.
    Offer { position: NonZeroUsize, perk: SmolStr },
    /// Controls the number of perks offered in the level up screen.
    NumberOfPerks {
        #[clap(subcommand)]
        subcommand: NumberOfPerksCommands,
    },
    /// Controls the reroll cost of the level up screen.
    RerollCost {
        #[clap(subcommand)]
        subcommand: RerollCostCommands,
    },
}

/// Number of perks commands.
#[derive(Debug, Subcommand)]
pub enum NumberOfPerksCommands {
    /// Shows the number of perks offered in the level up screen.
    Show,
    /// Sets the number of perks offered in the level up screen.
    Set { number_of_perks: u8 },
}

/// Reroll cost commands.
#[derive(Debug, Subcommand)]
pub enum RerollCostCommands {
    /// Shows the current reroll cost of the level up screen.
    Show,
    /// Controls the reroll cost model of the level up screen.
    Model {
        #[clap(subcommand)]
        subcommand: RerollCostModelCommands,
    },
}

/// Reroll cost model commands.
#[derive(Debug, Subcommand)]
pub enum RerollCostModelCommands {
    /// Shows the reroll cost model of the level up screen.
    Show,
    /// Sets the reroll cost model of the level up screen.
    Set {
        #[clap(subcommand)]
        subcommand: RerollCostModelSetCommands,
    },
}

/// Reroll cost model set commands.
#[derive(Debug, Subcommand)]
pub enum RerollCostModelSetCommands {
    /// Sets the reroll cost model of the level up screen to constant.
    #[clap(arg_required_else_help = true)]
    Constant { cost: f64 },
    /// Sets the reroll cost model of the level up screen to linear.
    #[clap(arg_required_else_help = true)]
    Linear {
        /// Sets the base price.
        #[clap(long)]
        base: f64,
        /// Sets increase per step.
        #[clap(long)]
        increase: f64,
        /// Sets the current step.
        #[clap(long)]
        step: Option<usize>,
        /// Sets the max price.
        #[clap(long)]
        max: Option<f64>,
    },
    /// Sets the reroll cost model of the level up screen to exponential.
    #[clap(arg_required_else_help = true)]
    Exponential {
        /// Sets the base price.
        #[clap(long)]
        base: f64,
        /// Sets increase factor per step.
        #[clap(long)]
        factor: f64,
        /// Sets the current step.
        #[clap(long)]
        step: Option<usize>,
        /// Sets the max price.
        #[clap(long)]
        max: Option<f64>,
    },
}
