use crate::{
    items::commands::*,
    prelude::*,
};


/// Applies the item console commands.
pub fn apply_command(mut command: ConsoleCommand<ItemCommand>, item_registry: Res<ItemRegistry>) {
    if let Some(Ok(ItemCommand { subcommand })) = command.take() {
        match subcommand {
            ItemCommands::List => {
                for (i, entry) in item_registry.iter().enumerate() {
                    reply!(command, "{}) {}", i + 1, entry.item.id());
                }
            },
        }
        reply!(command, "");
    }
}
