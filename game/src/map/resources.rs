use crate::prelude::*;


/// Resource for the bounds of the map.
#[derive(Debug, Reflect, Resource)]
pub struct MapBounds {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}
