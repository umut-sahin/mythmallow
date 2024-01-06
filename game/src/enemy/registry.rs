use crate::prelude::*;


/// Container for the enemy registry.
#[derive(Debug, Default, Deref, Resource)]
pub struct EnemyRegistry(Vec<EnemyRegistryEntry>);

impl EnemyRegistry {
    /// Registers an enemy to the enemy registry.
    pub fn register(&mut self, pack: impl IEnemyPack, enemy: impl IEnemy) -> &mut RegisteredEnemy {
        let pack_id = pack.id();
        let pack_index =
            self.iter().position(|entry| entry.pack.id() == pack_id).unwrap_or_else(|| {
                let index = self.len();
                self.0.push(EnemyRegistryEntry::new(pack));
                index
            });

        let enemies = &mut self.0[pack_index].enemies;

        let enemy_id = enemy.id();
        let enemy_index = match enemies.iter_mut().position(|enemy| enemy.id() == enemy_id) {
            Some(index) => {
                log::warn!(
                    "tried to register {:?} from {:?} enemy pack to the enemy registry again",
                    enemy_id,
                    pack_id,
                );
                index
            },
            None => {
                log::info!(
                    "registered {:?} from {:?} enemy pack to the enemy registry",
                    enemy_id,
                    pack_id,
                );
                let index = enemies.len();
                enemies.push(RegisteredEnemy::new(enemy));
                index
            },
        };

        &mut enemies[enemy_index]
    }
}

impl EnemyRegistry {
    /// Gets the number of enemy packs in the enemy registry.
    pub fn number_of_packs(&self) -> usize {
        self.0.len()
    }

    /// Gets the number of enemies in the enemy registry.
    pub fn number_of_enemies(&self) -> usize {
        self.0.iter().map(|entry| entry.enemies.len()).sum()
    }
}

impl Index<SelectedEnemyPackIndex> for EnemyRegistry {
    type Output = EnemyRegistryEntry;

    fn index(&self, enemy_pack_index: SelectedEnemyPackIndex) -> &EnemyRegistryEntry {
        &self.0[*enemy_pack_index]
    }
}


/// Container for the entries of the enemy registry.
#[derive(Debug)]
pub struct EnemyRegistryEntry {
    pub pack: RegisteredEnemyPack,
    pub enemies: Vec<RegisteredEnemy>,
}

impl EnemyRegistryEntry {
    /// Creates a new enemy registry entry.
    pub fn new(pack: impl IEnemyPack) -> EnemyRegistryEntry {
        EnemyRegistryEntry { pack: RegisteredEnemyPack::new(pack), enemies: Vec::new() }
    }
}

impl Deref for EnemyRegistryEntry {
    type Target = RegisteredEnemyPack;

    fn deref(&self) -> &RegisteredEnemyPack {
        &self.pack
    }
}


/// Container for registered enemy packs.
#[derive(Debug)]
pub struct RegisteredEnemyPack {
    pub pack: Arc<dyn IEnemyPack>,
}

impl RegisteredEnemyPack {
    /// Creates a new registered enemy pack.
    pub fn new(pack: impl IEnemyPack) -> RegisteredEnemyPack {
        RegisteredEnemyPack { pack: Arc::new(pack) }
    }
}

impl Deref for RegisteredEnemyPack {
    type Target = Arc<dyn IEnemyPack>;

    fn deref(&self) -> &Arc<dyn IEnemyPack> {
        &self.pack
    }
}


/// Container for registered enemies.
#[derive(Debug)]
pub struct RegisteredEnemy {
    pub enemy: Arc<dyn IEnemy>,
    pub tags: SmallVec<[SmolStr; 3]>,
}

impl RegisteredEnemy {
    /// Creates a new registered enemy.
    pub fn new(enemy: impl IEnemy) -> RegisteredEnemy {
        RegisteredEnemy { enemy: Arc::new(enemy), tags: SmallVec::new() }
    }
}

impl RegisteredEnemy {
    /// Adds a tag to the enemy.
    pub fn add_tag(&mut self, tag: impl ToString) -> &mut RegisteredEnemy {
        self.tags.push(tag.to_string().into());
        self
    }
}

impl RegisteredEnemy {
    /// Gets if the enemy has a tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|candidate| candidate == tag)
    }
}

impl Deref for RegisteredEnemy {
    type Target = Arc<dyn IEnemy>;

    fn deref(&self) -> &Arc<dyn IEnemy> {
        &self.enemy
    }
}
