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
}
