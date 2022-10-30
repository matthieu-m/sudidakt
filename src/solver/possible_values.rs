//! Meta-data keeping track of which values are possible in which position.

use crate::model::{DIMENSION, CellIndex, Digit, DigitCounter, DigitSet, Group};

/// Keeps track of which values are possible in which position.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PossibleValues {
    cells: [DigitSet; NUMBER_CELLS],
    group_counters: [DigitCounter; NUMBER_GROUPS],
}

impl PossibleValues {
    /// Returns an all-possible values.
    pub fn all() -> Self {
        Self { cells: [DigitSet::full(); NUMBER_CELLS], group_counters: [DigitCounter::full(); NUMBER_GROUPS] }
    }

    /// Returns the possible digits for a given position.
    pub fn of_cell(&self, cell: CellIndex) -> DigitSet { self.cells[cell.value()] }

    /// Returns the number of possibilities for each digit for a given group.
    pub fn of_group(&self, group: Group) -> DigitCounter { self.group_counters[Self::group_index(group)] }

    /// Resolves a digit for a given cell.
    ///
    /// Returns set of removed digits.
    ///
    /// Does not adjust the possible values of any _other_ cell.
    pub fn resolve(&mut self, cell: CellIndex, digit: Digit) -> DigitSet {
        assert!(self.of_cell(cell).has(digit),
            "Incoherent request. {:?} is not part of the possibilities of ({:?}, {:?}), only {:?}.",
            digit, cell.row(), cell.column(), self.of_cell(cell));

        let mut result = DigitSet::default();

        for possibility in self.of_cell(cell) {
            if possibility != digit {
                self.remove_possibility(cell, possibility);
                result.add(possibility);
            }
        }

        result
    }

    /// Removes a possible digit in a specific position.
    ///
    /// Returns the removed digit, if any.
    ///
    /// If the digit is already not possible, nothing happens.
    pub fn remove_possibility(&mut self, cell: CellIndex, digit: Digit) -> Option<Digit> {
        let slot = &mut self.cells[cell.value()];

        if !slot.has(digit) {
            return None;
        }

        slot.remove(digit);

        for group in Group::groups(cell) {
            self.group_counters[Self::group_index(group)].decrement(digit);
        }

        Some(digit)
    }
}

//
//  Implementation Details
//

const NUMBER_CELLS: usize = DIMENSION * DIMENSION;
const NUMBER_GROUPS: usize = 3 * DIMENSION;

impl PossibleValues {
    //  Internal: index of a group.
    fn group_index(group: Group) -> usize {
        match group {
            Group::Column(column) => column.value(),
            Group::Row(row) => DIMENSION + row.value(),
            Group::Square(square) => 2 * DIMENSION + square.value(),
        }
    }
}
