use crate::prelude::*;

/// Speedy perk which increases the speed of the player.
#[derive(Clone, Component, Copy, Debug, Reflect)]
pub struct Speedy {
    pub rarity: Rarity,
}

impl Speedy {
    pub fn delta_speed_multiplier(&self) -> f32 {
        0.03 * (self.rarity.level() as f32)
    }
}

impl IPerk for Speedy {
    fn id(&self) -> SmolStr {
        format_smolstr!("speedy-{}", self.rarity.id())
    }

    fn name(&self) -> LocalizedText {
        let rarity = self.rarity.name();
        LocalizedText::Localized {
            key: "speedy-name",
            args: smallvec![("rarity", rarity.into())],
            fallback: format!("Speedy {}", rarity).into(),
        }
    }

    fn description(&self) -> LocalizedText {
        let bonus = self.delta_speed_multiplier() * 100.00;
        LocalizedText::Localized {
            key: "speedy-description",
            args: smallvec![("bonus", format_smolstr!("{:.0}", bonus))],
            fallback: format!("+{:.0}% Speed", bonus).into(),
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
pub fn obtain(In(perk): In<Speedy>, mut player_query: Query<&mut SpeedMultiplier, With<Player>>) {
    if let Ok(mut player_speed_multiplier) = player_query.get_single_mut() {
        player_speed_multiplier.0 += perk.delta_speed_multiplier();
    }
}

/// Loses the perk.
pub fn lose(In(perk): In<Speedy>, mut player_query: Query<&mut SpeedMultiplier, With<Player>>) {
    if let Ok(mut player_speed_multiplier) = player_query.get_single_mut() {
        player_speed_multiplier.0 -= perk.delta_speed_multiplier();
    }
}
