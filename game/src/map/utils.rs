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
        Dir2::new(*direction).ok()?,
        distance,
        false,
        SpatialQueryFilter::from_mask([Layer::MapObstacle]),
    )
}
