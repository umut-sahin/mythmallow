use crate::prelude::*;

/// Controls the market.
#[derive(ConsoleCommand, Parser)]
#[command(name = "market")]
#[command(disable_help_flag = true)]
pub struct MarketCommand {
    #[clap(subcommand)]
    pub subcommand: MarketCommands,
}

/// Market commands.
#[derive(Debug, Subcommand)]
pub enum MarketCommands {
    /// Show the items offered in the market.
    Show,
    /// Refreshes the market.
    Refresh,
    /// Locks an item in the market.
    Lock { position: NonZeroUsize },
    /// Unlocks an item in the market.
    Unlock { position: NonZeroUsize },
    /// Acquires an item in the market.
    Acquire { position: NonZeroUsize },
    /// Opens the market.
    Open,
    /// Closes the market.
    Close,
    /// Controls the balance.
    Balance {
        #[clap(subcommand)]
        subcommand: BalanceCommands,
    },
    /// Controls the number of items in the market.
    NumberOfItems {
        #[clap(subcommand)]
        subcommand: NumberOfItemsCommands,
    },
    /// Controls the refresh cost of the market.
    RefreshCost {
        #[clap(subcommand)]
        subcommand: RefreshCostCommands,
    },
    /// Controls the number of free refreshes in the market.
    FreeRefreshes {
        #[clap(subcommand)]
        subcommand: FreeRefreshesCommands,
    },
}

/// Balance commands.
#[derive(Debug, Subcommand)]
pub enum BalanceCommands {
    /// Shows the balance.
    Show,
    /// Sets the balance.
    Set { amount: f64 },
    /// Increases the balance.
    Add { amount: f64 },
    /// Decreases the balance.
    Remove { amount: f64 },
}

/// Number of items commands.
#[derive(Debug, Subcommand)]
pub enum NumberOfItemsCommands {
    /// Shows the number of items offered in the market.
    Show,
    /// Sets the number of items offered in the market.
    Set { number_of_items: u8 },
}

/// Refresh cost commands.
#[derive(Debug, Subcommand)]
pub enum RefreshCostCommands {
    /// Shows the current refresh cost of the market.
    Show,
    /// Controls the refresh cost model of the market.
    Model {
        #[clap(subcommand)]
        subcommand: RefreshCostModelCommands,
    },
}

/// Refresh cost model commands.
#[derive(Debug, Subcommand)]
pub enum RefreshCostModelCommands {
    /// Shows the refresh cost model of the market.
    Show,
    /// Sets the refresh cost model of the market.
    Set {
        #[clap(subcommand)]
        subcommand: RefreshCostModelSetCommands,
    },
}

/// Refresh cost set commands.
#[derive(Debug, Subcommand)]
pub enum RefreshCostModelSetCommands {
    /// Sets the refresh cost model of the market to constant.
    #[clap(arg_required_else_help = true)]
    Constant { cost: f64 },
    /// Sets the refresh cost model of the market to linear.
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
    /// Sets the refresh cost model of the market to exponential.
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

/// Free refreshes commands.
#[derive(Debug, Subcommand)]
pub enum FreeRefreshesCommands {
    /// Shows the number of free refreshes in the market.
    Show,
    /// Sets the number of free refreshes in the market.
    Set { free_refreshes: usize },
}
