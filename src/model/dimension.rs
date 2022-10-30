//! The dimensions of the grid.

/// A typical 9x9 sudoku grid is sub-divided in 9 squares, of 3x3 cells.
pub const SQUARE_DIMENSION: usize = 3;

/// The entire grid width, or height.
pub const DIMENSION: usize = SQUARE_DIMENSION * SQUARE_DIMENSION;
