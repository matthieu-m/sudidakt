//! The various groups existing in a sudoku grid.

use crate::model::{DIMENSION, CellIndex, ColumnIndex, GroupIndex, RowIndex, SquareIndex};

/// A group of cells existing in sudoku.
///
/// The rules of the sudoku are defined by group: in each group, each value must appear exactly once.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Group {
    /// A column of the grid.
    Column(ColumnIndex),
    /// A row of the grid.
    Row(RowIndex),
    /// A square of the grid.
    Square(SquareIndex),
}

impl Group {
    /// Returns the group matching the index.
    pub fn new(group: GroupIndex) -> Group {
        if group.value() < DIMENSION {
            Group::Column(ColumnIndex::new(group.value()).unwrap())
        } else if group.value() < 2 * DIMENSION {
            Group::Row(RowIndex::new(group.value() - DIMENSION).unwrap())
        } else {
            debug_assert!(group.value() < 3 * DIMENSION);
            Group::Square(SquareIndex::new(group.value() - 2 * DIMENSION).unwrap())
        }
    }

    /// Returns the colum covering the specified cell.
    pub fn column(cell: CellIndex) -> Group { Group::Column(cell.column()) }

    /// Returns the row covering the specified cell.
    pub fn row(cell: CellIndex) -> Group { Group::Row(cell.row()) }

    /// Returns the square covering the specified cell.
    pub fn square(cell: CellIndex) -> Group { Group::Square(cell.square()) }

    /// Returns the 3 groups covering the specified cell.
    pub fn groups(cell: CellIndex) -> [Group; 3] {
        [Self::column(cell), Self::row(cell), Self::square(cell)]
    }

    /// Returns the index of the group.
    pub fn index(&self) -> GroupIndex {
        match *self {
            Group::Column(column) => GroupIndex::new(column.value()).unwrap(),
            Group::Row(row) => GroupIndex::new(row.value() + DIMENSION).unwrap(),
            Group::Square(square) => GroupIndex::new(square.value() + 2 * DIMENSION).unwrap(),
        }
    }

    /// Returns whether a cell is covered by the group.
    pub fn contains(&self, cell: CellIndex) -> bool {
        match *self {
            Group::Column(column) => column == cell.column(),
            Group::Row(row) => row == cell.row(),
            Group::Square(square) => square == cell.square(),
        }
    }

    /// Returns the indexes of all cells covered by the group.
    pub fn cells(&self) -> [CellIndex; DIMENSION] {
        let mut coordinates: [CellIndex; DIMENSION] = Default::default();

        match *self {
            Group::Column(column) => {
                for (index, row) in RowIndex::all().enumerate() {
                    coordinates[index] = CellIndex::from_coordinates(row, column);
                }
            },
            Group::Row(row) => {
                for (index, column) in ColumnIndex::all().enumerate() {
                    coordinates[index] = CellIndex::from_coordinates(row, column);
                }
            },
            Group::Square(square) => {
                let mut index = 0;

                for row in square.rows() {
                    for column in square.columns() {
                        coordinates[index] = CellIndex::from_coordinates(row, column);
                        index += 1;
                    }
                }
            },
        }

        coordinates
    }

    /// Returns the 2 other groups which cover the specified cell.
    ///
    /// #   Panics
    ///
    /// If the cell is not covered by the current group.
    pub fn other_groups(&self, cell: CellIndex) -> [Group; 2] {
        assert!(self.contains(cell),
            "Cell {:?} ({:?}, {:?}) is not covered by {:?}", cell, cell.row(), cell.column(), *self);

        match *self {
            Group::Column(_) => [Group::row(cell), Group::square(cell)],
            Group::Row(_) => [Group::column(cell), Group::square(cell)],
            Group::Square(_) => [Group::column(cell), Group::row(cell)],
        }
    }
}
