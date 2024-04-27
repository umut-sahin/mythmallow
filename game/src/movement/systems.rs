use crate::prelude::*;


/// Increases the velocity of the object that just started to dash.
pub fn start_dashing(mut query: Query<&mut LinearVelocity, Added<Dashing>>) {
    for mut velocity in &mut query {
        velocity.0 *= 3.00;
    }
}

/// Keeps dashing behavior until it finishes and decreases the velocity to what it was before.
pub fn keep_dashing(
    mut commands: Commands,
    time: Res<Time>,
    mut dashing_entities_query: Query<(Entity, &mut Dashing, &mut LinearVelocity)>,
) {
    for (entity, mut dashing, mut velocity) in &mut dashing_entities_query {
        dashing.timer.tick(time.delta());
        if dashing.timer.finished() {
            velocity.0 /= 3.00;
            commands.entity(entity).remove::<Dashing>();
        }
    }
}


/// Attracts objects to each other.
pub fn attraction(
    time: Res<Time>,
    mut attracted_query: Query<(
        &Position,
        &AttractedTo,
        &mut AttractionSpeed,
        Option<&IdealAttractionDistance>,
        Option<&SlowdownOfGoingBackwardsDuringAttraction>,
        &mut LinearVelocity,
    )>,
    target_query: Query<&Position>,
) {
    for (
        position,
        attracted_to,
        mut attraction_speed,
        ideal_distance,
        backwards_slowdown,
        mut velocity,
    ) in attracted_query.iter_mut()
    {
        if let Ok(target_position) = target_query.get(attracted_to.0) {
            let ideal_distance = *ideal_distance.cloned().unwrap_or_default();
            let direction = target_position.0 - position.0;

            let speed = match attraction_speed.deref_mut() {
                AttractionSpeed::Constant(speed) => *speed,
                AttractionSpeed::Accelerating {
                    min_speed,
                    acceleration_per_second,
                    current_speed,
                    max_speed,
                } => {
                    let previous_speed = *current_speed;

                    if current_speed.0 != max_speed.0 {
                        let new_speed =
                            current_speed.0 + (acceleration_per_second.0 * time.delta_seconds());
                        *current_speed = Speed(new_speed.clamp(min_speed.0, max_speed.0));
                    }

                    previous_speed
                },
            };

            velocity.0 = direction.normalize() * speed.0;
            if direction.length() < ideal_distance {
                velocity.0 *= -backwards_slowdown.cloned().unwrap_or_default().0;
            }
        }
    }
}
