use crate::prelude::*;


/// Tag component for the map.
#[derive(Component, Debug, Reflect)]
pub struct Map;


/// Tag component for the invisible walls around the map.
#[derive(Component, Debug, Reflect)]
pub struct MapBound;
