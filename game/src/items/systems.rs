use crate::{
    items::commands::*,
    prelude::*,
};


/// Applies the item console commands.
pub fn apply_command(item_registry: Res<ItemRegistry>, mut command: ConsoleCommand<ItemCommand>) {
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
