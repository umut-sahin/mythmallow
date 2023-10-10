use crate::prelude::*;


/// Increases the velocity of the object that just started to dash.
pub fn start_dashing(mut query: Query<&mut Velocity, Added<Dashing>>) {
    for mut velocity in &mut query {
        velocity.0 *= 3.00;
    }
}

/// Keeps dashing behavior until it finishes and decreases the velocity to what it was before.
pub fn keep_dashing(
    mut commands: Commands,
    time: Res<Time>,
    mut dashing_entities_query: Query<(Entity, &mut Dashing, &mut Velocity)>,
) {
    for (entity, mut dashing, mut velocity) in &mut dashing_entities_query {
        dashing.timer.tick(time.delta());
        if dashing.timer.finished() {
            velocity.0 /= 3.00;
            commands.entity(entity).remove::<Dashing>();
        }
    }
}
