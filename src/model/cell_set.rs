//! A set of cells.

use std::{convert, fmt, iter};

use super::{DIMENSION, CellIndex};

/// Set of CellIndex.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CellSet(u128);

impl CellSet {
    /// Creates an empty CellSet.
    pub fn empty() -> CellSet { CellSet::default() }

    /// Creates a full CellSet, with all values set.
    pub fn full() -> CellSet { CellSet((1 << NUMBER_CELLS) - 1) }

    /// Checks whether the set is empty.
    pub fn is_empty(&self) -> bool { self.0 == 0 }

    /// Returns the number of elements in the set.
    pub fn size(&self) -> usize { self.0.count_ones() as usize }

    /// Checks whether the set contains the indicated CellIndex.
    pub fn has(&self, cell: CellIndex) -> bool { (self.0 & Self::mask(cell)) != 0 }

    /// Adds the specified CellIndex.
    pub fn add(&mut self, cell: CellIndex) { self.0 |= Self::mask(cell) }

    /// Removes the specified CellIndex.
    pub fn remove(&mut self, cell: CellIndex) { self.0 &= !Self::mask(cell) }

    //  Internal: computes the index of a cell within the set.
    fn index(cell: CellIndex) -> usize { cell.value() }

    //  Internal: computes the bitmask with the only set bit being that of the specified cell.
    fn mask(cell: CellIndex) -> u128 { 1 << Self::index(cell) }
}

impl convert::From<CellIndex> for CellSet {
    fn from(cell: CellIndex) -> CellSet {
        let mut result = CellSet::default();
        result.add(cell);
        result
    }
}

impl fmt::Debug for CellSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_set().entries(self.into_iter()).finish()
    }
}

impl iter::IntoIterator for CellSet {
    type Item = CellIndex;
    type IntoIter = CellSetIterator;

    fn into_iter(self) -> Self::IntoIter { CellSetIterator(self.0) }
}

/// Iterator over a set of CellIndexs.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct CellSetIterator(u128);

impl iter::Iterator for CellSetIterator {
    type Item = CellIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let trailing = self.0.trailing_zeros();
        let mask = 1 << trailing;
        self.0 &= !mask;

        CellIndex::new(trailing as usize)
    }
}

//
//  Implementation
//

const NUMBER_CELLS: usize = DIMENSION * DIMENSION;

#[cfg(test)]
mod tests {

use super::*;

#[test]
fn empty_set() {
    let empty = CellSet::default();

    assert!(empty.is_empty());
    assert_eq!(0, empty.size());
    assert_eq!("{}", &format!("{:?}", empty));
}

#[test]
fn single_cell_set() {
    let single = CellSet::from(cell(3));

    assert!(!single.is_empty());
    assert_eq!(1, single.size());
    assert_eq!("{3}", &format!("{:?}", single));
}

#[test]
fn crud_cell_set() {
    let three = cell(3);
    let four = cell(4);

    let mut set = CellSet::default();
    set.add(three);

    assert!(set.has(three));
    assert!(!set.has(four));
    assert_eq!("{3}", &format!("{:?}", set));

    set.remove(four);

    assert!(set.has(three));
    assert!(!set.has(four));
    assert_eq!("{3}", &format!("{:?}", set));

    set.add(four);

    assert!(set.has(three));
    assert!(set.has(four));
    assert_eq!("{3, 4}", &format!("{:?}", set));

    set.remove(three);

    assert!(!set.has(three));
    assert!(set.has(four));
    assert_eq!("{4}", &format!("{:?}", set));

    set.remove(four);

    assert!(!set.has(three));
    assert!(!set.has(four));
    assert_eq!("{}", &format!("{:?}", set));
}

fn cell(cell: usize) -> CellIndex { CellIndex::new(cell).expect("Valid CellIndex") }

}
