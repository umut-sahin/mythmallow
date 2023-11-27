use crate::prelude::*;

/// Registry for players.
pub static PLAYER_REGISTRY: Mutex<PlayerRegistry> = Mutex::new(PlayerRegistry::new());

/// Container for player registry.
#[derive(Default, Deref, DerefMut, Resource)]
pub struct PlayerRegistry(pub Vec<(Arc<dyn Mythology>, Vec<Arc<dyn Playable>>)>);

impl PlayerRegistry {
    /// Creates a new player registry.
    pub const fn new() -> PlayerRegistry {
        PlayerRegistry(Vec::new())
    }
}

impl PlayerRegistry {
    /// Registers a player to player registry.
    pub fn register(&mut self, mythology: impl Mythology, player: impl Playable) {
        let mythology_name = mythology.name();
        let player_name = player.name();

        let entries = match self
            .iter_mut()
            .find(|(existing_mythology, _)| existing_mythology.id() == mythology.id())
        {
            Some((_, entries)) => entries,
            None => {
                self.push((Arc::new(mythology), Vec::new()));
                &mut self.last_mut().unwrap().1
            },
        };

        if entries.iter().any(|existing_player| existing_player.id() == player.id()) {
            log::warn!(
                "tried to register {:?} from {:?} mythology to player registry again",
                player_name,
                mythology_name,
            );
        } else {
            log::info!(
                "registered {:?} from {:?} mythology to player registry",
                player_name,
                mythology_name,
            );
            entries.push(Arc::new(player));
        }
    }
}

impl PlayerRegistry {
    /// Finds the player from it's id.
    pub fn find(&self, id: impl AsRef<str>) -> Option<PlayerIndex> {
        let id = id.as_ref();
        for (mythology_index, (_, players)) in self.iter().enumerate() {
            for (player_index, player) in players.iter().enumerate() {
                if player.id() == id {
                    return Some(PlayerIndex { mythology_index, player_index });
                }
            }
        }
        None
    }
}

impl Index<usize> for PlayerRegistry {
    type Output = (Arc<dyn Mythology>, Vec<Arc<dyn Playable>>);

    fn index(&self, index: usize) -> &(Arc<dyn Mythology>, Vec<Arc<dyn Playable>>) {
        &self.deref()[index]
    }
}

impl Index<PlayerIndex> for PlayerRegistry {
    type Output = Arc<dyn Playable>;

    fn index(&self, index: PlayerIndex) -> &Arc<dyn Playable> {
        let (_, players) = &self.deref()[index.mythology_index];
        &players.deref()[index.player_index]
    }
}
