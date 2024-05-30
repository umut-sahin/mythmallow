use crate::{
    perk::constants::DEFAULT_PERK_COMMONNESS,
    prelude::*,
};


/// Interface for perks.
pub trait IPerk: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the perk.
    fn id(&self) -> SmolStr;
    /// Gets the name of the perk.
    fn name(&self) -> SmolStr;
    /// Gets the description of the perk.
    fn description(&self) -> SmolStr;

    /// Gets the rarity of the perk.
    fn rarity(&self) -> Rarity;
    /// Gets the commonness of the perk in the power up screen.
    fn commonness(&self) -> u64 {
        let mut commonness = DEFAULT_PERK_COMMONNESS;
        for _ in 1..(self.rarity() as u8) {
            commonness /= 3;
        }
        commonness
    }

    /// Obtains the perk.
    fn obtain(&self, world: &mut World);
    /// Loses the perk.
    fn lose(&self, world: &mut World);
}
