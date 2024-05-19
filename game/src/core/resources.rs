use crate::prelude::*;


/// Database of registered systems.
#[derive(Debug, Resource)]
pub struct RegisteredSystems {
    pub leveling: RegisteredLevelingSystems,
    pub market: RegisteredMarketSystems,
}

impl RegisteredSystems {
    /// Creates the database.
    pub fn new(app: &mut App) -> RegisteredSystems {
        let systems = app.world.spawn(Name::new("RegisteredSystems")).id();
        RegisteredSystems {
            leveling: RegisteredLevelingSystems::new(app, systems),
            market: RegisteredMarketSystems::new(app, systems),
        }
    }

    /// Attaches a system into systems entity.
    pub fn attach<I>(
        app: &mut App,
        systems: Entity,
        system: SystemId<I>,
        name: impl Into<Cow<'static, str>>,
    ) {
        unsafe {
            // This is safe as long as SystemId<I> is just an Entity in runtime.
            // And transmute wouldn't work if SystemId<I> and Entity don't have the same size.
            let system = std::mem::transmute::<SystemId<I>, Entity>(system);
            if let Some(mut systems) = app.world.get_entity_mut(systems) {
                systems.add_child(system);
                if let Some(mut system) = app.world.get_entity_mut(system) {
                    system.insert(Name::new(name));
                }
            }
        }
    }
}


/// State stack for the game state.
#[derive(Debug, Default, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct GameStateStack(pub Vec<GameState>);

impl GameStateStack {
    /// Transitions to a new game state.
    pub fn transition(&mut self, state: GameState) {
        self.0.pop();
        self.0.push(state);
    }
}


/// Result of the game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect, Resource)]
pub enum GameResult {
    Won,
    Lost,
}
