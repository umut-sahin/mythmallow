use crate::prelude::*;


/// Resource for the index of the selected enemy pack.
#[derive(Clone, Copy, Debug, Deref, Reflect, Resource)]
pub struct SelectedEnemyPackIndex(pub usize);


/// Resource for the selected enemy pack.
#[derive(Clone, Debug, Deref, Resource)]
pub struct SelectedEnemyPack(pub (Arc<dyn IEnemyPack>, Vec<EnemyRegistryEntry>));


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
#[derive(Clone, Resource)]
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

impl Debug for EnemySpawnPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut m = f.debug_map();
        for (i, spawn) in self.spawns.lock().unwrap().iter().enumerate() {
            m.entry(&i, &spawn);
        }
        m.finish()
    }
}


/// Details of the enemy spawn.
pub struct EnemySpawn {
    /// Delay for the first spawn.
    pub delay: Timer,
    /// Enemy to spawn.
    pub enemy: Arc<dyn IEnemy>,
    /// Group size.
    pub count: u32,
    /// Optional spawn interval within the group.
    pub interval: Option<Timer>,
    /// Group position.
    pub position: EnemySpawnPosition,
    /// Group direction.
    pub direction: EnemySpawnDirection,
    /// Group spread.
    pub spread: EnemySpawnSpread,
    /// Optional repeat for the spawn.
    pub repeat: Option<Timer>,

    /// Amount of enemies spawned within the group.
    ///
    /// Also used for retrying a failed spawn.
    pub(crate) spawned: u32,

    /// Amount of enemies to be spawned within the group.
    ///
    /// Can be used for multiple groups.
    pub(crate) remaining: u32,

    /// Current spawn position for the group.
    ///
    /// Updated on every `self.count` spawns.
    pub(crate) group_position: Position,
}

impl EnemySpawn {
    /// Creates a new enemy spawn.
    pub fn new(delay: Duration, enemy: &Arc<dyn IEnemy>) -> EnemySpawn {
        EnemySpawn {
            delay: Timer::new(delay, TimerMode::Once),
            enemy: Arc::clone(enemy),
            count: 1,
            interval: None,
            position: EnemySpawnPosition::Random,
            direction: EnemySpawnDirection::any(),
            spread: EnemySpawnSpread::default(),
            repeat: None,
            spawned: 0,
            remaining: 0,
            group_position: Position::new(Vector::ZERO),
        }
    }

    /// Sets the group size of the spawn.
    ///
    /// # Panics
    ///
    /// - Panics if `count` is zero.
    pub fn count(mut self, count: u32) -> EnemySpawn {
        if count == 0 {
            panic!("spawn count cannot be 0");
        }
        self.count = count;
        self
    }

    /// Sets the interval of the spawn.
    pub fn interval(mut self, interval: Duration) -> EnemySpawn {
        self.interval = Some(Timer::new(interval, TimerMode::Repeating));
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

impl Debug for EnemySpawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Pattern");
        s.field("delay", &self.delay.duration());
        s.field("enemy", &self.enemy);
        s.field("count", &self.count);
        if let Some(timer) = self.interval.as_ref() {
            s.field("interval", &timer.duration());
        }
        s.field("position", &self.position);
        s.field("direction", &self.direction);
        s.field("spread", &self.spread);
        if let Some(timer) = self.repeat.as_ref() {
            s.field("repeat", &timer.duration());
        }
        s.finish()
    }
}


/// Position for the enemy spawn.
#[derive(Clone, Copy, Debug)]
pub enum EnemySpawnPosition {
    /// In a predefined position. If set, spawn direction is ignored.
    At(Position),
    /// Within certain distance to the player.
    AroundPlayer { near: f32, far: f32 },
    /// Random across the whole map.
    Random,
}


/// Direction for the enemy spawn.
#[derive(Clone, Copy, Debug)]
pub struct EnemySpawnDirection {
    pub from_degrees: f32,
    pub to_degrees: f32,
}

impl EnemySpawnDirection {
    pub fn any() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 0.00, to_degrees: 360.00 }
    }

    pub fn top() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 0.00, to_degrees: 180.00 }
    }

    pub fn bottom() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 180.00, to_degrees: 360.00 }
    }

    pub fn left() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 90.00, to_degrees: 270.00 }
    }

    pub fn right() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 270.00, to_degrees: 90.00 }
    }

    pub fn top_left() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 90.00, to_degrees: 180.00 }
    }

    pub fn top_right() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 0.00, to_degrees: 90.00 }
    }

    pub fn bottom_left() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 180.00, to_degrees: 270.00 }
    }

    pub fn bottom_right() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 270.00, to_degrees: 360.00 }
    }
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
