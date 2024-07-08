use crate::prelude::*;

/// Regenerates the health of entities that have health regeneration.
pub fn hp_regeneration(
    time: Res<Time>,
    mut query: Query<(&HpRegeneration, &Health, &mut RemainingHealth)>,
) {
    for (hp_regeneration, health, mut remaining_health) in query.iter_mut() {
        let health_to_regenerate = hp_regeneration.0.max(0.00) * time.delta_seconds();
        remaining_health.0 += health_to_regenerate;

        if remaining_health.0 > health.0 {
            remaining_health.0 = health.0;
        }
    }
}
