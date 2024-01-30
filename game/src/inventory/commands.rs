use crate::prelude::*;

/// Controls the inventory.
#[derive(ConsoleCommand, Parser)]
#[command(name = "inventory")]
#[command(disable_help_flag = true)]
pub struct InventoryCommand {
    #[clap(subcommand)]
    pub subcommand: InventoryCommands,
}

#[derive(Debug, Subcommand)]
pub enum InventoryCommands {
    /// Lists the items in the inventory.
    List,
    /// Adds an item to the inventory.
    Add { item: String },
}
