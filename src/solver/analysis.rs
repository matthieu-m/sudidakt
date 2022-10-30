//! Home to the various analyses of a given Sudoku grid.
//!
//! The analyses are about using the grid and the meta-information about the grid to further infer more
//! meta-information.

/// The total number of analyses.
pub const NUMBER_ANALYSIS: usize = ALL_ANALYSES.len();

/// All analyses, from cheapest to most expensive.
pub const ALL_ANALYSES: [Analysis; 5] = [
    Analysis::CellExclusion, Analysis::GroupExclusion, Analysis::GroupInclusion, Analysis::GroupOverlap,
    Analysis::GroupSubsetInclusion,
];

/// The various analyses algorithms, from cheap to expensive.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Analysis {
    /// A single digit may appear in a given cell, hence when a digit is placed, no other digit remains possible.
    CellExclusion = 0,
    /// A given digit may only appear once in any group, hence when a digit is known to be in a specific cell, none of
    /// the other cells covered by the same 3 groups can possibly contain this digit.
    ///
    /// #   Algorithmic Complexity
    ///
    /// Constant in space and time.
    GroupExclusion,
    /// A given digit must appear at least once in any group, hence when a digit is only possible in one of the cells
    /// covered by a group, this cell must resolve to this digit.
    ///
    /// #   Algorithmic Complexity
    ///
    /// Constant in space and time.
    GroupInclusion,
    /// A square and a line overlap in a set of 3 cells. If for either of the 2 groups, square or line, a given digit
    /// must appear in this overlap, then it cannot appear outside of this overlap for the other group.
    ///
    /// #   Algorithmic Complexity
    ///
    /// Linear (time) in the number of cells in a group.
    GroupOverlap,
    /// When a set of N digits is the only set of possible values for a set of N cells within a given group, then those
    /// N digits can only appear within those N cells.
    ///
    /// This is an extension of the GroupInclusion analysis, to a degree, although it is much more costly to run.
    ///
    /// #   Algorithmic Complexity
    ///
    /// Cubic (time) in the number of cells in a group.
    GroupSubsetInclusion,
}
