use crate::prelude::*;

/// Component for rarity of items/perks.
#[derive(Clone, Component, Copy, Debug, EnumIter, Reflect)]
#[repr(u8)]
pub enum Rarity {
    Common = 1,
    Rare = 2,
    Epic = 3,
    Legendary = 4,
}

impl Rarity {
    /// Gets the identifier of the rarity.
    pub fn id(&self) -> &'static str {
        match self {
            Rarity::Common => "i",
            Rarity::Rare => "ii",
            Rarity::Epic => "iii",
            Rarity::Legendary => "iv",
        }
    }

    /// Gets the name of the rarity.
    pub fn name(&self) -> &'static str {
        match self {
            Rarity::Common => "I",
            Rarity::Rare => "II",
            Rarity::Epic => "III",
            Rarity::Legendary => "IV",
        }
    }

    /// Gets the level of the rarity.
    pub fn level(&self) -> u8 {
        *self as u8
    }
}

impl Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_smolstr!("{:?}", self).to_lowercase())
    }
}
