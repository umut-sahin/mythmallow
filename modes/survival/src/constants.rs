use mythmallow::prelude::*;


/// Size of the grid. Value of 10 means the map will be a 10x10 grid of squares.
pub const GRID_SIZE: i32 = 10;

/// Amount of space between grid elements.
pub const GRID_SPACING: f32 = 50.0;

/// Thickness of grid elements.
pub const GRID_WIDTH: f32 = 2.0;

/// Color of the grid.
pub const GRID_COLOR: Color = Color::rgb(0.27, 0.27, 0.27);


/// Size of the map.
pub const MAP_SIZE: f32 = (GRID_SIZE as f32) * GRID_SPACING;

/// Bounds of the map.
pub const MAP_BOUNDS: MapBounds = MapBounds {
    x_min: -(MAP_SIZE / 2.00),
    x_max: (MAP_SIZE / 2.00),
    y_min: -(MAP_SIZE / 2.00),
    y_max: (MAP_SIZE / 2.00),
};
