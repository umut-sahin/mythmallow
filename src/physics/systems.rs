use crate::{
    physics::{
        constants::*,
        schedules::*,
    },
    prelude::*,
};


/// Collects collisions into collisions resource.
pub fn collect_collisions(
    query: Query<(Entity, &Position, &Velocity, &Collider)>,
    mut collisions: ResMut<Collisions>,
) {
    collisions.clear();

    // safety margin multiplier bigger than 1
    // to account for sudden accelerations
    let k = 2.0;

    let safety_margin_factor = k * DELTA_TIME;
    let safety_margin_factor_squared = safety_margin_factor * safety_margin_factor;

    unsafe {
        for (entity_a, position_a, velocity_a, collider_a) in query.iter_unsafe() {
            let velocity_a_squared = velocity_a.length_squared();
            for (entity_b, position_b, velocity_b, collider_b) in query.iter_unsafe() {
                if entity_a <= entity_b {
                    continue;
                }

                let velocity_b_squared = velocity_b.length_squared();
                let safety_margin_squared =
                    safety_margin_factor_squared * (velocity_a_squared + velocity_b_squared);

                let ab = position_b.0 - position_a.0;
                let ab_length_squared = ab.length_squared();

                let combined_radius = collider_a.radius + collider_b.radius;
                let combined_radius_squared = combined_radius * combined_radius;

                let combined_radius_with_safety_margin =
                    combined_radius + safety_margin_squared.sqrt();
                let combined_radius_with_safety_margin_squared =
                    combined_radius_with_safety_margin * combined_radius_with_safety_margin;

                if ab_length_squared < combined_radius_with_safety_margin_squared {
                    let overlapping = ab_length_squared < combined_radius_squared;
                    collisions.push(Collision::new(entity_a, entity_b).overlapping(overlapping));
                }
            }
        }
    }
}

/// Runs the substeps schedule.
pub fn run_substeps(world: &mut World) {
    for _ in 0..SUBSTEPS {
        world.run_schedule(SubstepSchedule);
    }
}

/// Confines entities to the map area.
pub fn confine_entities(
    map_bounds: Option<Res<MapBounds>>,
    mut query: Query<(&mut Position, &Collider)>,
) {
    if let Some(map_bounds) = map_bounds {
        for (mut position, collider) in &mut query {
            let bottom_left = position.0 - collider.radius;
            let top_right = position.0 + collider.radius;

            if top_right.x > map_bounds.x_max {
                position.x = map_bounds.x_max - collider.radius;
            } else if bottom_left.x < map_bounds.x_min {
                position.x = map_bounds.x_min + collider.radius;
            }

            if top_right.y > map_bounds.y_max {
                position.y = map_bounds.y_max - collider.radius;
            } else if bottom_left.y < map_bounds.y_min {
                position.y = map_bounds.y_min + collider.radius;
            }
        }
    }
}

/// Updates translation of physics entities based on their positions.
pub fn update_transforms(mut query: Query<(&mut Transform, &Position), Changed<Position>>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}


/// Updates position of physics entities based on their velocities.
pub fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0 * SUBSTEP_DELTA_TIME;
    }
}

/// Updates position of physics entities based on collisions.
pub fn resolve_collisions(
    query: Query<(&Collider, &mut Position), Without<Floating>>,
    collisions: ResMut<Collisions>,
) {
    for collision in collisions.iter() {
        let (entity_a, entity_b) = collision.entities;
        assert_ne!(entity_a, entity_b);

        let (collider_a, mut position_a) = unsafe {
            match query.get_unchecked(entity_a) {
                Ok(components) => components,
                Err(_) => continue,
            }
        };
        let (collider_b, mut position_b) = unsafe {
            match query.get_unchecked(entity_b) {
                Ok(components) => components,
                Err(_) => continue,
            }
        };

        let ab = position_b.0 - position_a.0;
        let ab_length_squared = ab.length_squared();

        let combined_radius = collider_a.radius + collider_b.radius;
        let combined_radius_squared = combined_radius * combined_radius;

        if ab_length_squared < combined_radius_squared {
            let ab_length = ab_length_squared.sqrt();

            let ab_normalized = ab / ab_length;
            let penetration_depth = combined_radius - ab_length;

            position_a.0 -= ab_normalized * penetration_depth * 0.50;
            position_b.0 += ab_normalized * penetration_depth * 0.50;
        }
    }
}
