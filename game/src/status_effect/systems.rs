use crate::prelude::*;


/// Reduces, and eventually clears, the cooldown.
pub fn cooldown<T: Send + Sync + 'static>(
    mut commands: Commands,
    time: Res<Time>,
    mut cooldown_query: Query<(Entity, &mut Cooldown<T>)>,
) {
    for (entity, mut cooldown) in cooldown_query.iter_mut() {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.finished() {
            commands.entity(entity).remove::<Cooldown<T>>();
        }
    }
}
