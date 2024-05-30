use crate::prelude::*;


/// Container for the perk registry.
#[derive(Debug, Default, Deref, Resource)]
pub struct PerkRegistry(Vec<PerkRegistryEntry>);

impl PerkRegistry {
    /// Registers a perk to the perk registry.
    pub fn register(&mut self, perk: impl IPerk) -> &mut RegisteredPerk {
        let perk_id = perk.id();
        let perk_index = match self.iter().position(|entry| entry.perk.id() == perk_id) {
            Some(index) => {
                log::warn!("tried to register {:?} to the perk registry again", perk_id);
                index
            },
            None => {
                log::info!("registered {:?} to the perk registry", perk_id);
                let index = self.len();
                self.0.push(PerkRegistryEntry::new(perk));
                index
            },
        };
        &mut self.0[perk_index].perk
    }
}

impl PerkRegistry {
    /// Gets the number of perks in the perk registry.
    pub fn number_of_perks(&self) -> usize {
        self.0.len()
    }

    /// Finds the perk with the specified id.
    pub fn find_perk_by_id(&self, perk_id: &str) -> Option<&RegisteredPerk> {
        for entry in self.iter() {
            if entry.perk.id() == perk_id {
                return Some(&entry.perk);
            }
        }
        None
    }

    /// Finds the perk with the specified id mutably.
    pub fn find_perk_mut_by_id(&mut self, perk_id: &str) -> Option<&mut RegisteredPerk> {
        for entry in self.0.iter_mut() {
            if entry.perk.id() == perk_id {
                return Some(&mut entry.perk);
            }
        }
        None
    }
}


/// Container for the entries of the iem registry.
#[derive(Debug)]
pub struct PerkRegistryEntry {
    pub perk: RegisteredPerk,
}

impl PerkRegistryEntry {
    /// Creates a new perk registry entry.
    pub fn new(perk: impl IPerk) -> PerkRegistryEntry {
        PerkRegistryEntry { perk: RegisteredPerk::new(perk) }
    }
}


/// Container for registered perks.
#[derive(Clone, Debug)]
pub struct RegisteredPerk {
    pub perk: Arc<dyn IPerk>,
    pub description: SmolStr,
    pub rarity: Rarity,
    pub commonness: u64,
}

impl RegisteredPerk {
    /// Creates a new registered perk.
    pub fn new(perk: impl IPerk) -> RegisteredPerk {
        let description = perk.description();
        let rarity = perk.rarity();
        let commonness = perk.commonness();
        RegisteredPerk { perk: Arc::new(perk), description, rarity, commonness }
    }
}

impl Deref for RegisteredPerk {
    type Target = Arc<dyn IPerk>;

    fn deref(&self) -> &Arc<dyn IPerk> {
        &self.perk
    }
}
