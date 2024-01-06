use crate::prelude::*;


/// Finds the enemies in range from a position ordered by their distance.
pub fn find_enemies_in_range(
    spatial_query: &SpatialQuery,
    position: &Position,
    area: &Collider,
    enemy_hit_box_query: &Query<&Position, With<EnemyHitBox>>,
) -> Vec<(Entity, Position, f32)> {
    let intersections = spatial_query.shape_intersections(
        area,
        position.xy(),
        0.00,
        SpatialQueryFilter::new().with_masks([Layer::EnemyHitBox]),
    );

    let mut enemies_in_range = intersections
        .iter()
        .filter_map(|&enemy_hit_box_entity| {
            enemy_hit_box_query
                .get(enemy_hit_box_entity)
                .map(|&enemy_hit_box_position| {
                    (
                        enemy_hit_box_entity,
                        enemy_hit_box_position,
                        enemy_hit_box_position.distance(position.xy()),
                    )
                })
                .ok()
        })
        .collect::<Vec<_>>();

    enemies_in_range.sort_by(|(_, _, distance1), (_, _, distance2)| {
        distance1.partial_cmp(distance2).unwrap_or(Ordering::Equal)
    });

    enemies_in_range
}
