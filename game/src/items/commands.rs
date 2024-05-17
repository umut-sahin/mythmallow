use crate::prelude::*;

/// Controls the items.
#[derive(ConsoleCommand, Parser)]
#[command(name = "item")]
#[command(disable_help_flag = true)]
pub struct ItemCommand {
    #[clap(subcommand)]
    pub subcommand: ItemCommands,
}

#[derive(Debug, Subcommand)]
pub enum ItemCommands {
    /// Lists the available items.
    List,
    /// Controls the base price of items.
    BasePrice {
        #[clap(subcommand)]
        subcommand: ItemBasePriceCommands,
    },
    /// Controls the commonness of items.
    Commonness {
        #[clap(subcommand)]
        subcommand: ItemCommonnessCommands,
    },
}

/// Item base price commands.
#[derive(Debug, Subcommand)]
pub enum ItemBasePriceCommands {
    /// Shows the base price of an item.
    #[clap(arg_required_else_help = true)]
    Show { item: SmolStr },
    /// Sets the base price of an item.
    #[clap(arg_required_else_help = true)]
    Set { item: SmolStr, base_price: f64 },
}

/// Item base price commands.
#[derive(Debug, Subcommand)]
pub enum ItemCommonnessCommands {
    /// Shows the commonness of an item.
    #[clap(arg_required_else_help = true)]
    Show { item: SmolStr },
    #[clap(arg_required_else_help = true)]
    /// Sets the commonness of an item.
    Set { item: SmolStr, commonness: u64 },
}
