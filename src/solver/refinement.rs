//! A refinement to the set of possible values of a cell.

use crate::model::{CellIndex, CellSet, Digit, DigitSet, Group};

/// A refinement to the set of possible values of a cell.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Refinement {
    cell: CellIndex,
    removed: Digit,
    reason: RefinementReason,
}

impl Refinement {
    /// Creates an instance.
    pub fn new(cell: CellIndex, removed: Digit, reason: RefinementReason) -> Self { Self { cell, removed, reason, } }

    /// Returns the cell in which the digit was removed.
    pub fn cell(&self) -> CellIndex { self.cell }

    /// Returns the new set of possibilities.
    pub fn removed(&self) -> Digit { self.removed }

    /// Returns the reason the removal occurred.
    pub fn reason(&self) -> RefinementReason { self.reason }
}

/// The reason for which a refinement occurred.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RefinementReason {
    /// CellExclusion: the digit which was placed in the cell.
    CellExclusion(Digit),
    /// Group Exclusion: the cell in which the removed digit was set, and the group guiding the removal.
    GroupExclusion(CellIndex, Group),
    /// Group Inclusion: the digit which was placed in the cell, and the group guiding the removal.
    GroupInclusion(Digit, Group),
    /// GroupOverlap: the group for which the digit is only present in the overlap, and the group guiding the removal.
    GroupOverlap(Group, Group),
    /// GroupSubsetInclusion: subset of cells and digits, and the group guiding the removal.
    GroupSubsetInclusion(CellSet, DigitSet, Group),
}
