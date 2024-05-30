use crate::prelude::*;


/// Database of registered perk systems.
#[derive(Clone, Copy, Debug, Resource)]
pub struct RegisteredPerkSystems {
    pub obtain_perk: SystemId<(Arc<dyn IPerk>, ObtainLosePerkReason)>,
    pub lose_perk: SystemId<(Arc<dyn IPerk>, ObtainLosePerkReason)>,
}

impl RegisteredPerkSystems {
    /// Creates the database.
    pub fn new(app: &mut App, systems: Entity) -> RegisteredPerkSystems {
        use super::systems::*;

        let obtain_perk = app.world.register_system(obtain_perk);
        RegisteredSystems::attach(app, systems, obtain_perk, "obtain_perk");

        let lose_perk = app.world.register_system(lose_perk);
        RegisteredSystems::attach(app, systems, lose_perk, "lose_perk");

        RegisteredPerkSystems { obtain_perk, lose_perk }
    }
}


/// Reason for obtaining/losing a perk.
#[derive(Debug)]
pub enum ObtainLosePerkReason {
    LevelingUp { to: Level },
    LevelingDown { to: Level },
    Cheating,
    OppositeOfCheating,
}

impl Display for ObtainLosePerkReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObtainLosePerkReason::LevelingUp { to } => {
                write!(f, "for leveling up to level {}", to.get())
            },
            ObtainLosePerkReason::LevelingDown { to } => {
                write!(f, "for leveling down to level {}", to.get())
            },
            ObtainLosePerkReason::Cheating => write!(f, "by cheating :)"),
            ObtainLosePerkReason::OppositeOfCheating => write!(f, "by cheating :|"),
        }
    }
}
