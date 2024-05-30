use crate::prelude::*;


/// Event for obtaining a perk.
#[derive(Debug, Event)]
pub struct PerkObtainedEvent {
    pub perk: Arc<dyn IPerk>,
    pub reason: ObtainLosePerkReason,
}


/// Event for losing a perk.
#[derive(Debug, Event)]
pub struct PerkLostEvent {
    pub perk: Arc<dyn IPerk>,
    pub reason: ObtainLosePerkReason,
}
