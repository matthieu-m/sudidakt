//! The placement of a digit in a cell, taking a firm step towards completion.

use crate::model::{CellIndex, Digit};

/// Placement, and its meta-information.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Placement {
    cell: CellIndex,
    digit: Digit,
}

impl Placement {
    /// Creates an instance.
    pub fn new(cell: CellIndex, digit: Digit) -> Self { Self { cell, digit, } }

    /// Returns the cell in which a digit was placed.
    pub fn cell(&self) -> CellIndex { self.cell }

    /// Returns the digit placed in the cell.
    pub fn digit(&self) -> Digit { self.digit }
}
