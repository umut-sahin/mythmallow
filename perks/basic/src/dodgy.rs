use crate::prelude::*;

/// Dodgy perk which increases the dodge chance of the player.
#[derive(Clone, Component, Copy, Debug, Reflect)]
pub struct Dodgy {
    pub rarity: Rarity,
}

impl Dodgy {
    pub fn delta_dodge_chance(&self) -> f32 {
        3.00 * (self.rarity.level() as f32)
    }
}

impl IPerk for Dodgy {
    fn id(&self) -> SmolStr {
        format_smolstr!("dodgy-{}", self.rarity.id())
    }

    fn name(&self) -> LocalizedText {
        let rarity = self.rarity.name();
        LocalizedText::Localized {
            key: "dodgy-name",
            args: smallvec![("rarity", rarity.into())],
            fallback: format!("Dodgy {}", rarity).into(),
        }
    }

    fn description(&self) -> LocalizedText {
        let bonus = self.delta_dodge_chance();
        LocalizedText::Localized {
            key: "dodgy-description",
            args: smallvec![("bonus", format_smolstr!("{:.0}", bonus))],
            fallback: format!("+{:.0}% Dodge Chance", bonus).into(),
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
pub fn obtain(In(perk): In<Dodgy>, mut player_query: Query<&mut DodgeChance, With<Player>>) {
    if let Ok(mut player_dodge_chance) = player_query.get_single_mut() {
        player_dodge_chance.0 += perk.delta_dodge_chance();
    }
}

/// Loses the perk.
pub fn lose(In(perk): In<Dodgy>, mut player_query: Query<&mut DodgeChance, With<Player>>) {
    if let Ok(mut player_dodge_chance) = player_query.get_single_mut() {
        player_dodge_chance.0 -= perk.delta_dodge_chance();
    }
}
