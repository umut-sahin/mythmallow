use crate::prelude::*;


/// Finds the first obstacle from `position` along `direction` within `distance` units.
pub fn find_obstacle(
    spatial_query: &SpatialQuery,
    position: &Position,
    direction: &Vec2,
    distance: f32,
) -> Option<RayHitData> {
    spatial_query.cast_ray(
        position.xy(),
        *direction,
        distance,
        false,
        SpatialQueryFilter::new().with_masks([Layer::MapObstacle]),
    )
}
