use crate::prelude::*;

/// Registry for enemies.
pub static ENEMY_REGISTRY: Mutex<EnemyRegistry> = Mutex::new(EnemyRegistry::new());

/// Container for enemy registry.
#[derive(Default, Deref, DerefMut, Resource)]
pub struct EnemyRegistry(pub Vec<(Arc<dyn MunchiePack>, Vec<EnemyRegistryEntry>)>);

impl EnemyRegistry {
    /// Creates a new enemy registry.
    pub const fn new() -> EnemyRegistry {
        EnemyRegistry(Vec::new())
    }
}

impl EnemyRegistry {
    /// Registers an enemy to enemy registry.
    pub fn register(
        &mut self,
        pack: impl MunchiePack,
        enemy: impl Munchie,
    ) -> &mut EnemyRegistryEntry {
        let pack_id = pack.id();
        let pack_name = pack.name();

        let pack_index = match self.iter_mut().position(|(candidate, _)| candidate.id() == pack_id)
        {
            Some(pack_index) => pack_index,
            None => {
                let pack_index = self.len();
                self.push((Arc::new(pack), Vec::new()));
                pack_index
            },
        };
        let pack_entries = &mut self.0[pack_index].1;

        let enemy_id = enemy.id();
        let enemy_name = enemy.name();

        let enemy_index =
            match pack_entries.iter_mut().position(|candidate| candidate.id() == enemy_id) {
                Some(enemy_index) => {
                    log::warn!(
                        "tried to register {:?} from {:?} enemy pack to enemy registry again",
                        enemy_name,
                        pack_name,
                    );

                    enemy_index
                },
                None => {
                    log::info!(
                        "registered {:?} from {:?} enemy pack to enemy registry",
                        enemy_name,
                        pack_name,
                    );

                    let enemy_index = pack_entries.len();
                    pack_entries.push(EnemyRegistryEntry::new(enemy));
                    enemy_index
                },
            };

        &mut pack_entries[enemy_index]
    }
}

impl Index<SelectedEnemyPackIndex> for EnemyRegistry {
    type Output = (Arc<dyn MunchiePack>, Vec<EnemyRegistryEntry>);

    fn index(
        &self,
        index: SelectedEnemyPackIndex,
    ) -> &(Arc<dyn MunchiePack>, Vec<EnemyRegistryEntry>) {
        &self.deref()[index.0]
    }
}

/// Container for enemy registry entries.
#[derive(Clone, Debug)]
pub struct EnemyRegistryEntry {
    pub enemy: Arc<dyn Munchie>,
    pub tags: SmallVec<[SmolStr; 3]>,
}

impl EnemyRegistryEntry {
    /// Create a new entry for an enemy.
    pub fn new<E: Munchie>(enemy: E) -> EnemyRegistryEntry {
        EnemyRegistryEntry { enemy: Arc::new(enemy), tags: SmallVec::new() }
    }
}

impl EnemyRegistryEntry {
    /// Add a tag to the item.
    pub fn add_tag(&mut self, tag: impl ToString) -> &mut EnemyRegistryEntry {
        self.tags.push(tag.to_string().into());
        self
    }
}

impl EnemyRegistryEntry {
    /// Gets if the enemy has the tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|candidate| candidate == tag)
    }
}

impl Deref for EnemyRegistryEntry {
    type Target = Arc<dyn Munchie>;

    fn deref(&self) -> &Arc<dyn Munchie> {
        &self.enemy
    }
}
