use crate::prelude::*;

/// Speedy perk which increases the health of the player.
#[derive(Clone, Component, Copy, Debug, Reflect)]
pub struct Healthy {
    pub rarity: Rarity,
}

impl Healthy {
    pub fn health_bonus(&self) -> f32 {
        3.00 * (self.rarity.level() as f32)
    }
}

impl IPerk for Healthy {
    fn id(&self) -> SmolStr {
        format_smolstr!("healthy-{}", self.rarity.id())
    }

    fn name(&self) -> SmolStr {
        format_smolstr!("Healthy {}", self.rarity.name())
    }

    fn description(&self) -> SmolStr {
        format_smolstr!("+{} Health", self.health_bonus())
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
    In(perk): In<Healthy>,
    mut player_query: Query<(&mut Health, &mut RemainingHealth), With<Player>>,
) {
    if let Ok((mut player_health, mut player_remaining_health)) = player_query.get_single_mut() {
        player_health.0 += perk.health_bonus();
        player_remaining_health.0 += perk.health_bonus();
    }
}

/// Loses the perk.
pub fn lose(
    In(perk): In<Healthy>,
    mut player_query: Query<(&mut Health, &mut RemainingHealth), With<Player>>,
) {
    if let Ok((mut player_health, mut player_remaining_health)) = player_query.get_single_mut() {
        player_health.0 -= perk.health_bonus();
        player_remaining_health.0 -= perk.health_bonus();
    }
}
