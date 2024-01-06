use crate::prelude::*;


/// Container for the player registry.
#[derive(Debug, Default, Deref, Resource)]
pub struct PlayerRegistry(Vec<PlayerRegistryEntry>);

impl PlayerRegistry {
    /// Registers a player to the player registry.
    pub fn register(
        &mut self,
        mythology: impl IMythology,
        player: impl IPlayer,
    ) -> &mut RegisteredPlayer {
        let mythology_id = mythology.id();
        let mythology_index = self
            .iter()
            .position(|entry| entry.mythology.id() == mythology_id)
            .unwrap_or_else(|| {
                let index = self.len();
                self.0.push(PlayerRegistryEntry::new(mythology));
                index
            });

        let players = &mut self.0[mythology_index].players;

        let player_id = player.id();
        let player_index = match players
            .iter_mut()
            .position(|registered_player| registered_player.id() == player_id)
        {
            Some(index) => {
                log::warn!(
                    "tried to register {:?} from {:?} mythology to the player registry again",
                    player_id,
                    mythology_id,
                );
                index
            },
            None => {
                log::info!(
                    "registered {:?} from {:?} mythology to the player registry",
                    player_id,
                    mythology_id,
                );
                let index = players.len();
                players.push(RegisteredPlayer::new(player));
                index
            },
        };

        &mut players[player_index]
    }
}

impl PlayerRegistry {
    /// Gets the number of mythologies in the player registry.
    pub fn number_of_mythologies(&self) -> usize {
        self.len()
    }

    /// Gets the number of players in the player registry.
    pub fn number_of_players(&self) -> usize {
        self.iter().map(|entry| entry.players.len()).sum()
    }
}

impl PlayerRegistry {
    /// Tries to find the player in the player registry by id.
    pub fn find_player(
        &self,
        player_id: impl AsRef<str>,
    ) -> Option<(SelectedMythologyIndex, SelectedPlayerIndex)> {
        let player_id = player_id.as_ref();
        for (mythology_index, entry) in self.iter().enumerate() {
            for (player_index, player) in entry.players.iter().enumerate() {
                if player.id() == player_id {
                    return Some((
                        SelectedMythologyIndex(mythology_index),
                        SelectedPlayerIndex(player_index),
                    ));
                }
            }
        }
        None
    }
}

impl Index<SelectedMythologyIndex> for PlayerRegistry {
    type Output = PlayerRegistryEntry;

    fn index(&self, mythology_index: SelectedMythologyIndex) -> &PlayerRegistryEntry {
        &self.0[*mythology_index]
    }
}


/// Container for the entries of the player registry.
#[derive(Debug)]
pub struct PlayerRegistryEntry {
    pub mythology: RegisteredMythology,
    pub players: Vec<RegisteredPlayer>,
}

impl PlayerRegistryEntry {
    /// Creates a new player registry entry.
    pub fn new(mythology: impl IMythology) -> PlayerRegistryEntry {
        PlayerRegistryEntry { mythology: RegisteredMythology::new(mythology), players: Vec::new() }
    }
}

impl Deref for PlayerRegistryEntry {
    type Target = RegisteredMythology;

    fn deref(&self) -> &RegisteredMythology {
        &self.mythology
    }
}

impl Index<SelectedPlayerIndex> for PlayerRegistryEntry {
    type Output = RegisteredPlayer;

    fn index(&self, player_index: SelectedPlayerIndex) -> &RegisteredPlayer {
        &self.players[*player_index]
    }
}


/// Container for registered mythologies.
#[derive(Debug)]
pub struct RegisteredMythology {
    pub mythology: Arc<dyn IMythology>,
}

impl RegisteredMythology {
    /// Creates a new registered mythology.
    pub fn new(mythology: impl IMythology) -> RegisteredMythology {
        RegisteredMythology { mythology: Arc::new(mythology) }
    }
}

impl Deref for RegisteredMythology {
    type Target = Arc<dyn IMythology>;

    fn deref(&self) -> &Arc<dyn IMythology> {
        &self.mythology
    }
}


/// Container for registered players.
#[derive(Debug)]
pub struct RegisteredPlayer {
    pub player: Arc<dyn IPlayer>,
}

impl RegisteredPlayer {
    /// Creates a new registered player.
    pub fn new(player: impl IPlayer) -> RegisteredPlayer {
        RegisteredPlayer { player: Arc::new(player) }
    }
}

impl Deref for RegisteredPlayer {
    type Target = Arc<dyn IPlayer>;

    fn deref(&self) -> &Arc<dyn IPlayer> {
        &self.player
    }
}
