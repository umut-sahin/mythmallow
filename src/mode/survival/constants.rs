/// Size of the map. Value of 10 means the map will be a 10x10 grid of squares.
pub const MAP_SIZE: i32 = 10;


/// Amount of space between grid elements.
pub const GRID_SPACING: f32 = 50.0;

/// Thickness of grid elements.
pub const GRID_WIDTH: f32 = 2.0;


/// Bound of the map in terms of world coordinates.
pub const MAP_BOUND: f32 = (MAP_SIZE as f32) * GRID_SPACING / 2.00;
