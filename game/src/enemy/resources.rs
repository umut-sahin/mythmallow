use crate::prelude::*;


/// Resource for the index of the selected enemy pack.
#[derive(Clone, Copy, Debug, Deref, Reflect, Resource)]
pub struct SelectedEnemyPackIndex(pub usize);


/// Resource for the selected enemy pack.
#[derive(Clone, Debug, Deref, Resource)]
pub struct SelectedEnemyPack(pub (Arc<dyn MunchiePack>, Vec<EnemyRegistryEntry>));


/// Resource for counting spawned enemies.
#[derive(Debug, Default, Reflect, Resource)]
pub struct EnemyCounter(usize);

impl EnemyCounter {
    /// Gets the spawned enemy count.
    pub fn get(&self) -> usize {
        self.0
    }

    /// Increments the spawned enemy count.
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}


/// Resource for the enemy spawn pattern for the selected game mode and the selected enemy pack.
#[derive(Clone, Debug, Resource)]
pub struct EnemySpawnPattern {
    /// Spawns in the spawn pattern.
    pub spawns: Arc<Mutex<Vec<EnemySpawn>>>,
}

impl EnemySpawnPattern {
    /// Creates a new enemy spawn pattern.
    pub fn new(spawns: Vec<EnemySpawn>) -> EnemySpawnPattern {
        EnemySpawnPattern { spawns: Arc::new(Mutex::new(spawns)) }
    }
}


/// Details of the enemy spawn.
#[derive(Debug)]
pub struct EnemySpawn {
    /// Enemy to spawn.
    pub enemy: Arc<dyn Munchie>,
    /// Group size.
    pub count: u32,
    /// Group position.
    pub position: EnemySpawnPosition,
    /// Group direction.
    pub direction: EnemySpawnDirection,
    /// Group spread.
    pub spread: EnemySpawnSpread,
    /// Delay for the first spawn.
    pub delay: Timer,
    /// Optional repeat for the spawn.
    pub repeat: Option<Timer>,
}

impl EnemySpawn {
    /// Creates a new enemy spawn.
    pub fn new(enemy: &Arc<dyn Munchie>, delay: Duration) -> EnemySpawn {
        EnemySpawn {
            enemy: Arc::clone(enemy),
            count: 1,
            position: EnemySpawnPosition::Random,
            direction: EnemySpawnDirection::Any,
            spread: EnemySpawnSpread::default(),
            delay: Timer::new(delay, TimerMode::Once),
            repeat: None,
        }
    }

    /// Sets the group size of the spawn.
    pub fn count(mut self, count: u32) -> EnemySpawn {
        self.count = count;
        self
    }

    /// Sets the group position of the spawn.
    pub fn position(mut self, position: EnemySpawnPosition) -> EnemySpawn {
        self.position = position;
        self
    }

    /// Sets the group direction of the spawn.
    pub fn direction(mut self, direction: EnemySpawnDirection) -> EnemySpawn {
        self.direction = direction;
        self
    }

    /// Sets the group spread of the spawn.
    pub fn spread(mut self, spread: EnemySpawnSpread) -> EnemySpawn {
        self.spread = spread;
        self
    }

    /// Sets the repeat of the spawn.
    pub fn repeat(mut self, repeat: Duration) -> EnemySpawn {
        self.repeat = Some(Timer::new(repeat, TimerMode::Repeating));
        self
    }
}


/// Position for the enemy spawn.
#[derive(Debug)]
pub enum EnemySpawnPosition {
    /// In a predefined position. If set, spawn direction is ignored.
    At(Position),
    /// Within certain distance to the player.
    AroundPlayer { near: f32, far: f32 },
    /// Random across the whole map.
    Random,
}


/// Direction for the enemy spawn.
#[derive(Debug)]
pub enum EnemySpawnDirection {
    /// Spawn the enemy in any direction.
    Any,
    /// Tries to spawn the enemy above the player.
    Above,
    /// Tries to spawn the enemy below the player.
    Below,
    /// Tries to spawn the enemy left of the player.
    Left,
    /// Tries to spawn the enemy right of the player.
    Right,
    /// Tries to spawn the enemy between two angles.
    Between(Rotation, Rotation),
}


/// Spread of the enemy spawn.
#[derive(Debug, Default)]
pub struct EnemySpawnSpread {
    /// Minimum distance on the x axis from the base position.
    pub x_min: f32,
    /// Maximum distance on the x axis from the base position.
    pub x_max: f32,
    /// Minimum distance on the y axis from the base position.
    pub y_min: f32,
    /// Maximum distance on the y axis from the base position.
    pub y_max: f32,
}

impl EnemySpawnSpread {
    /// Creates a square spread.
    pub fn square(size: f32) -> EnemySpawnSpread {
        let spread = size / 2.00;
        EnemySpawnSpread { x_min: -spread, x_max: spread, y_min: -spread, y_max: spread }
    }
}
