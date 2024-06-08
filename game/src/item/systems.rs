use crate::{
    item::commands::*,
    prelude::*,
};


/// Applies the item console commands.
pub fn apply_item_command(
    mut item_registry: ResMut<ItemRegistry>,
    localization: Res<Localization>,
    mut command: ConsoleCommand<ItemCommand>,
) {
    if let Some(Ok(ItemCommand { subcommand })) = command.take() {
        match subcommand {
            ItemCommands::List => {
                for (i, entry) in item_registry.iter().enumerate() {
                    let item = &entry.item;
                    reply!(command, "{}) {}", i + 1, item.id());
                    reply!(command, "    - name: {}", item.name().get(&localization));
                    reply!(command, "    - tags: {}", item.tags.iter().join(", "));
                    reply!(command, "    - base price: {}", item.base_price);
                    reply!(
                        command,
                        "    - commonness: {:}",
                        item.commonness.to_formatted_string(&NumLocale::es_US),
                    );
                }
            },
            ItemCommands::BasePrice { subcommand } => {
                match subcommand {
                    ItemBasePriceCommands::Show { item } => {
                        match item_registry.find_item_by_id(&item) {
                            Some(item) => {
                                reply!(command, "{}", item.base_price);
                            },
                            None => {
                                reply!(command, "Item doesn't exist.");
                            },
                        }
                    },
                    ItemBasePriceCommands::Set { item, base_price } => {
                        match item_registry.find_item_mut_by_id(&item) {
                            Some(item) => {
                                log::info!(
                                    "setting the base price of {:?} to {}",
                                    item.id(),
                                    base_price,
                                );
                                item.base_price = Balance(base_price);
                                reply!(command, "Done.");
                            },
                            None => {
                                reply!(command, "Item doesn't exist.");
                            },
                        }
                    },
                }
            },
            ItemCommands::Commonness { subcommand } => {
                match subcommand {
                    ItemCommonnessCommands::Show { item } => {
                        match item_registry.find_item_by_id(&item) {
                            Some(item) => {
                                reply!(
                                    command,
                                    "{}",
                                    item.commonness.to_formatted_string(&NumLocale::es_US),
                                );
                            },
                            None => {
                                reply!(command, "Item doesn't exist.");
                            },
                        }
                    },
                    ItemCommonnessCommands::Set { item, commonness } => {
                        match item_registry.find_item_mut_by_id(&item) {
                            Some(item) => {
                                log::info!(
                                    "setting the commonness of {:?} to {}",
                                    item.id(),
                                    commonness.to_formatted_string(&NumLocale::es_US),
                                );
                                item.commonness = commonness;
                                reply!(command, "Done.");
                            },
                            None => {
                                reply!(command, "Item doesn't exist.");
                            },
                        }
                    },
                }
            },
        }
        reply!(command, "");
    }
}
