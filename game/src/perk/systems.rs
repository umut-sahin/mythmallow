use crate::{
    perk::commands::*,
    prelude::*,
};


/// Applies the perk console commands.
pub fn apply_perk_command(
    mut commands: Commands,
    mut perk_registry: ResMut<PerkRegistry>,
    registered_systems: Res<RegisteredSystems>,
    mut command: ConsoleCommand<PerkCommand>,
) {
    if let Some(Ok(PerkCommand { subcommand })) = command.take() {
        match subcommand {
            PerkCommands::List => {
                for (i, entry) in perk_registry.iter().enumerate() {
                    let perk = &entry.perk;
                    reply!(command, "{}) {}", i + 1, perk.id());
                    reply!(command, "    - description: {}", perk.description);
                    reply!(command, "    - rarity: {}", perk.rarity);
                    reply!(
                        command,
                        "    - commonness: {}",
                        perk.commonness.to_formatted_string(&Locale::es_US),
                    );
                }
            },
            PerkCommands::Obtain { perk } => {
                match perk_registry.find_perk_by_id(&perk) {
                    Some(perk) => {
                        commands.run_system_with_input(
                            registered_systems.perk.obtain_perk,
                            (perk.deref().clone(), ObtainLosePerkReason::Cheating),
                        );
                        reply!(command, "Obtained.");
                    },
                    None => {
                        reply!(command, "Failed to obtain {:?} perk as it doesn't exist.", perk);
                        reply!(command, "Run \"perk list\" to see available perks.")
                    },
                }
            },
            PerkCommands::Lose { perk } => {
                match perk_registry.find_perk_by_id(&perk) {
                    Some(perk) => {
                        commands.run_system_with_input(
                            registered_systems.perk.lose_perk,
                            (perk.deref().clone(), ObtainLosePerkReason::OppositeOfCheating),
                        );
                        reply!(command, "Lost.");
                    },
                    None => {
                        reply!(command, "Failed to lose {:?} perk as it doesn't exist.", perk);
                        reply!(command, "Run \"perk list\" to see available perks.")
                    },
                }
            },
            PerkCommands::Commonness { subcommand } => {
                match subcommand {
                    PerkCommonnessCommands::Show { perk } => {
                        match perk_registry.find_perk_by_id(&perk) {
                            Some(perk) => {
                                reply!(
                                    command,
                                    "{}",
                                    perk.commonness.to_formatted_string(&Locale::es_US),
                                );
                            },
                            None => {
                                reply!(command, "Perk doesn't exist.");
                            },
                        }
                    },
                    PerkCommonnessCommands::Set { perk, commonness } => {
                        match perk_registry.find_perk_mut_by_id(&perk) {
                            Some(perk) => {
                                log::info!(
                                    "setting the commonness of {:?} perk to {}",
                                    perk.id(),
                                    commonness.to_formatted_string(&Locale::es_US),
                                );
                                perk.commonness = commonness;
                                reply!(command, "Done.");
                            },
                            None => {
                                reply!(command, "Perk doesn't exist.");
                            },
                        }
                    },
                }
            },
        }
        reply!(command, "");
    }
}


/// Obtains a perk.
pub fn obtain_perk(
    In((perk, reason)): In<(Arc<dyn IPerk>, ObtainLosePerkReason)>,
    world: &mut World,
) {
    log::info!("obtaining {:?} perk {}", perk.id(), reason);
    perk.obtain(world);

    let mut perk_obtained_events = world.resource_mut::<Events<PerkObtainedEvent>>();
    perk_obtained_events.send(PerkObtainedEvent { perk, reason });
}

/// Loses a perk.
pub fn lose_perk(
    In((perk, reason)): In<(Arc<dyn IPerk>, ObtainLosePerkReason)>,
    world: &mut World,
) {
    log::info!("losing {:?} perk {}", perk.id(), reason);
    perk.lose(world);

    let mut perk_lost_events = world.resource_mut::<Events<PerkLostEvent>>();
    perk_lost_events.send(PerkLostEvent { perk, reason });
}
