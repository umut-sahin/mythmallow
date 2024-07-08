use crate::prelude::*;

/// Regenerative perk which increases the HP regeneration of the player.
#[derive(Clone, Component, Copy, Debug, Reflect)]
pub struct Regenerative {
    pub rarity: Rarity,
}

impl Regenerative {
    pub fn delta_hp_regeneration(&self) -> f32 {
        match self.rarity {
            Rarity::Common => 0.10,
            Rarity::Rare => 0.30,
            Rarity::Epic => 0.50,
            Rarity::Legendary => 1.00,
        }
    }
}

impl IPerk for Regenerative {
    fn id(&self) -> SmolStr {
        format_smolstr!("regenerative-{}", self.rarity.id())
    }

    fn name(&self) -> LocalizedText {
        let rarity = self.rarity.name();
        LocalizedText::Localized {
            key: "regenerative-name",
            args: smallvec![("rarity", rarity.into())],
            fallback: format!("Regenerative {}", rarity).into(),
        }
    }

    fn description(&self) -> LocalizedText {
        let bonus = self.delta_hp_regeneration();
        LocalizedText::Localized {
            key: "regenerative-description",
            args: smallvec![("bonus", format_smolstr!("{:.2}", bonus))],
            fallback: format!("+{:.2} HP Regeneration / Seconds", bonus).into(),
        }
    }

    fn rarity(&self) -> Rarity {
        self.rarity
    }

    fn obtain(&self, world: &mut World) {
        world.run_system_once_with(*self, obtain);
    }

    fn lose(&self, world: &mut World) {
        world.run_system_once_with(*self, lose);
    }
}

/// Obtains the perk.
pub fn obtain(
    In(perk): In<Regenerative>,
    mut player_query: Query<&mut HpRegeneration, With<Player>>,
) {
    if let Ok(mut player_hp_regeneration) = player_query.get_single_mut() {
        player_hp_regeneration.0 += perk.delta_hp_regeneration();
    }
}

/// Loses the perk.
pub fn lose(
    In(perk): In<Regenerative>,
    mut player_query: Query<&mut HpRegeneration, With<Player>>,
) {
    if let Ok(mut player_hp_regeneration) = player_query.get_single_mut() {
        player_hp_regeneration.0 -= perk.delta_hp_regeneration();
    }
}
