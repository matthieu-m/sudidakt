//! The `Grid` is the simplest representation of the sudoku grid.

use std::mem;

use super::{DIMENSION, CellIndex, Digit, Group};

/// A `Grid` object, mostly used for input and output.
///
/// A `Grid` models a sudoku grid of `Grid::DIMENSION` by `Grid::DIMENSION`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Grid(u8, [Option<Digit>; NUMBER_CELLS]);

impl Grid {
    /// Creates an empty grid.
    pub fn new() -> Grid { Grid::default() }

    /// Returns the number of known digits.
    pub fn number_solved(&self) -> usize { self.0 as usize }

    /// Returns the digit at the specified indexes, or none if the digit is not yet known.
    pub fn get_digit(&self, cell: CellIndex) -> Option<Digit> { self.1[cell.value()] }

    /// Returns the index of a conflicting cell, if any.
    ///
    /// Conflicts are determined as per the rules of sudoku, that is another cell sharing a row, column, or square
    /// with the same digit.
    pub fn get_conflicting(&self, cell: CellIndex, digit: Digit) -> Option<CellIndex> {
        for group in Group::groups(cell) {
            for candidate in group.cells() {
                if candidate != cell && self.get_digit(candidate) == Some(digit) {
                    return Some(candidate);
                }
            }
        }

        None
    }

    /// Sets the digit at the specified index.
    ///
    /// Returns the previously set digit, if any.
    pub fn set_digit(&mut self, cell: CellIndex, digit: Option<Digit>) -> Option<Digit> {
        let previous = mem::replace(&mut self.1[cell.value()], digit);

        if previous.is_some() && digit.is_none() {
            self.0 -= 1;
        } else if previous.is_none() && digit.is_some() {
            self.0 += 1;
        }

        previous
    }
}

impl Default for Grid {
    fn default() -> Self { Grid(0, [None; NUMBER_CELLS]) }
}

//
//  Implementation Details
//

const NUMBER_CELLS: usize = DIMENSION * DIMENSION;
