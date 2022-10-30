//! Indexes for the grid.

use std::{fmt, iter, marker::PhantomData, ops::Range};

use super::{DIMENSION, dimension::SQUARE_DIMENSION};

const NUMBER_CELLS: usize = DIMENSION * DIMENSION;
const NUMBER_GROUPS: usize = 3 * DIMENSION;

/// Tag of `ColumnIndex`.
#[doc(hidden)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColumnTag;

/// Index of a column of the grid, 0-based.
pub type ColumnIndex = Index<ColumnTag, DIMENSION>;

/// Tag of `RowIndex`.
#[doc(hidden)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RowTag;

/// Index of a column of the grid, 0-based.
pub type RowIndex = Index<RowTag, DIMENSION>;

/// Tag of `CellIndex`.
#[doc(hidden)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CellTag;

/// Index of a cell of the grid, 0-based.
///
/// Aribitrarily, the first cells are (0,0), (0,1), ...
pub type CellIndex = Index<CellTag, NUMBER_CELLS>;

impl Index<CellTag, NUMBER_CELLS> {
    /// Creates an instance from the row and column indexes.
    pub fn from_coordinates(row: RowIndex, column: ColumnIndex) -> Self {
        let row_offset = DIMENSION * row.value();

        Self((row_offset + column.value()) as u8, PhantomData)
    }

    /// Returns the column index of the cell.
    pub fn column(&self) -> ColumnIndex {
        let index = self.value() % DIMENSION;

        Index(index as u8, PhantomData)
    }

    /// Returns the row index of the cell.
    pub fn row(&self) -> RowIndex {
        let index = self.value() / DIMENSION;

        Index(index as u8, PhantomData)
    }

    /// Returns the square index of the cell.
    pub fn square(&self) -> SquareIndex { SquareIndex::from_coordinates(self.row(), self.column()) }
}

/// Tag of `SquareIndex`.
#[doc(hidden)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SquareTag;

/// Index of a square of the grid, 0-based.
pub type SquareIndex = Index<SquareTag, DIMENSION>;

impl Index<SquareTag, DIMENSION> {
    /// Creates an instance from the row and column indexes.
    pub fn from_coordinates(row: RowIndex, column: ColumnIndex) -> Self {
        let row_offset = SQUARE_DIMENSION * (row.value() / SQUARE_DIMENSION);
        let column_offset = column.value() / SQUARE_DIMENSION;

        let index = row_offset + column_offset;
        debug_assert!(index < DIMENSION);

        Index(index as u8, PhantomData)
    }

    /// Creates an iterator over all the columns covered by the square.
    pub fn columns(&self) -> IndexRangeIterator<ColumnTag, DIMENSION> {
        let start_column = SQUARE_DIMENSION * (self.value() % SQUARE_DIMENSION);
        let end_column = start_column + SQUARE_DIMENSION;

        IndexRangeIterator::new(start_column..end_column).expect("Within bounds")
    }

    /// Creates an iterator over all the rows covered by the square.
    pub fn rows(&self) -> IndexRangeIterator<RowTag, DIMENSION> {
        let start_row = SQUARE_DIMENSION * (self.value() / SQUARE_DIMENSION);
        let end_row = start_row + SQUARE_DIMENSION;

        IndexRangeIterator::new(start_row..end_row).expect("Within bounds")
    }
}

/// Tag of `GroupIndex`.
#[doc(hidden)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GroupTag;

/// Index of a square of the grid, 0-based.
pub type GroupIndex = Index<GroupTag, NUMBER_GROUPS>;

/// A generic Index type.
///
/// The `Tag` allows differentiating multiple indexes with the same other characteristics, it is expected to be a
/// zero-sized value. An index can only take values within the `0..BOUND` range.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Index<Tag, const BOUND: usize>(u8, Invariant<Tag>);

impl<T, const B: usize> Index<T, B> {
    /// Creates an iterator which iterates over the entire range of possible values of the index.
    pub fn all() -> IndexRangeIterator<T, B> { IndexRangeIterator::new(0..B).expect("Within bounds") }

    /// Creates an index from the specified value, if within bounds.
    pub fn new(value: usize) -> Option<Index<T, B>> {
        (value < B)
            .then(|| u8::try_from(value).ok().map(|u| Index(u, PhantomData)))
            .flatten()
    }

    /// Returns the actual value of the index.
    pub fn value(&self) -> usize { self.0.into() }
}

impl<T, const B: usize> fmt::Debug for Index<T, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.0)
    }
}

/// An iterator over an index.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct IndexRangeIterator<Tag, const BOUND: usize>(Range<usize>, Invariant<Tag>);

impl<T, const B: usize> IndexRangeIterator<T, B> {
    /// Creates a new range iterator.
    pub fn new(range: Range<usize>) -> Option<Self> {
        if range.start <= B && range.end <= B {
            Some(IndexRangeIterator(range, PhantomData))
        } else {
            None
        }
    }
}

impl<T, const B: usize> iter::Iterator for IndexRangeIterator<T,  B> {
    type Item = Index<T, B>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .and_then(|n| u8::try_from(n).ok())
            .map(|u| Index(u, PhantomData))
    }
}

//
//  Implementation details
//

type Invariant<T> = PhantomData<fn(T)->T>;


#[cfg(test)]
mod tests {

use super::*;

#[test]
fn valid_columns() {
    for i in 0..DIMENSION {
        assert_eq!(i, ColumnIndex::new(i).expect("Valid").value());
    }
}

#[test]
fn invalid_column() {
    assert_eq!(None, ColumnIndex::new(DIMENSION));
}

#[test]
fn valid_rows() {
    for i in 0..DIMENSION {
        assert_eq!(i, RowIndex::new(i).expect("Valid").value());
    }
}

#[test]
fn invalid_row() {
    assert_eq!(None, RowIndex::new(DIMENSION));
}

#[test]
fn valid_square() {
    for i in 0..DIMENSION {
        assert_eq!(i, SquareIndex::new(i).expect("Valid").value());
    }
}

#[test]
fn invalid_square() {
    assert_eq!(None, SquareIndex::new(DIMENSION));
}

const COLUMNS_PER_SQUARE: [Range<usize>; DIMENSION] = [0..3, 3..6, 6..9, 0..3, 3..6, 6..9, 0..3, 3..6, 6..9];
const ROWS_PER_SQUARE: [Range<usize>; DIMENSION] = [0..3, 0..3, 0..3, 3..6, 3..6, 3..6, 6..9, 6..9, 6..9];

#[test]
fn square_from_row_column() {
    for (index, (rows, columns)) in ROWS_PER_SQUARE.iter().zip(COLUMNS_PER_SQUARE.iter()).enumerate() {
        for row in rows.clone() {
            for column in columns.clone() {
                let row = RowIndex::new(row).unwrap();
                let column = ColumnIndex::new(column).unwrap();

                let square = SquareIndex::from_coordinates(row, column);

                assert_eq!(index, square.value(), "Row: {row:?}, Column: {column:?}");
            }
        }
    }
}

#[test]
fn square_columns() {
    for square_index in SquareIndex::all() {
        assert_eq!(SQUARE_DIMENSION, square_index.columns().count());

        let expected = COLUMNS_PER_SQUARE[square_index.value()].clone();

        for (expected, actual) in expected.into_iter().zip(square_index.columns()) {
            assert_eq!(expected, actual.value());
        }
    }
}

#[test]
fn square_rows() {
    for square_index in SquareIndex::all() {
        assert_eq!(SQUARE_DIMENSION, square_index.rows().count());

        let expected = ROWS_PER_SQUARE[square_index.value()].clone();

        for (expected, actual) in expected.into_iter().zip(square_index.rows()) {
            assert_eq!(expected, actual.value());
        }
    }
}

#[test]
fn cell_column_row() {
    for row in 0..DIMENSION {
        for column in 0..DIMENSION {
            let cell = CellIndex::new(row * DIMENSION + column).unwrap();

            assert_eq!(column, cell.column().value(), "Row: {row}, Column: {column}");
            assert_eq!(row, cell.row().value(), "Row: {row}, Column: {column}");
        }
    }
}

} // mod tests
