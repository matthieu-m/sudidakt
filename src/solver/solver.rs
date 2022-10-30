//! The Solver itself.

use std::{error, fmt};

use crate::model::{DIMENSION, CellIndex, Digit, Grid};
use super::{Analyzer, JournalReader, JournalWriter, Placement, Placer, PossibleValues, Refinement};

/// The didactic solver assistant.
#[derive(Clone, Debug)]
pub struct Solver {
    grid: Grid,
    analyzer: Analyzer,
    placer: Placer,
}

impl Solver {
    /// Creates a new instance of Solver from a given Grid.
    pub fn new(grid: Grid) -> Solver {
        let placements = JournalWriter::new();

        let analyzer = Analyzer::new(placements.reader());
        let mut placer = Placer::new(placements, analyzer.refinements());

        for cell in CellIndex::all() {
            if let Some(digit) = grid.get_digit(cell) {
                placer.set_digit(cell, digit);
            }
        }

        Solver { grid, analyzer, placer, }
    }

    /// Returns the current grid.
    pub fn grid(&self) -> Grid { self.grid }

    /// Returns the current possible values.
    pub fn possible_values(&self) -> PossibleValues { self.analyzer.possible_values() }

    /// Returns the successive set of refinements performed by the solver.
    pub fn refinements(&self) -> JournalReader<Refinement> { self.analyzer.refinements() }

    /// Returns the successive set of placements performed by the solver, or user.
    pub fn placements(&self) -> JournalReader<Placement> { self.placer.placements() }

    /// Sets the specified digit in the specified cell, if allowed.
    ///
    /// Returns an error if the digit is not allowed as per the rules of Sudoku.
    pub fn set_digit(&mut self, cell: CellIndex, digit: Digit) -> Result<(), ConflictError> {
        //  Check for conflicts before any modification.
        if let Some(conflicting) = self.grid.get_conflicting(cell, digit) {
            return Err(ConflictError{ digit, candidate: cell, conflicting });
        }

        //  Mark as resolved.
        self.grid.set_digit(cell, Some(digit));
        self.placer.set_digit(cell, digit);

        Ok(())
    }

    /// Solves the grid completely, if possible.
    ///
    /// Returns an error if no progress can be made.
    pub fn solve(&mut self) -> Result<(), ProgressStalled> {
        while self.grid.number_solved() < DIMENSION * DIMENSION {
            #[cfg(debug_assertions)]
            eprintln!("Solver::solve - {} cells solved so far", self.grid.number_solved());

            if let Ok(_) = self.place() {
                continue;
            }

            if let Ok(_) = self.refine() {
                continue;
            }

            return Err(ProgressStalled{});
        }

        Ok(())
    }

    /// Places the next value, if possible.
    ///
    /// Returns whether any placement occurred, or not.
    pub fn place(&mut self) -> Result<(), ProgressStalled> {
        while !self.placer.is_done() {
            if let Some(placement) = self.placer.handle_next_refinement() {
                self.grid.set_digit(placement.cell(), Some(placement.digit()));

                return Ok(());
            }
        }

        Err(ProgressStalled{})
    }

    /// Refines the set of possible values, if possible.
    ///
    /// Returns whether any refinement occurred, or not.
    pub fn refine(&mut self) -> Result<(), ProgressStalled> {
        while !self.analyzer.is_done() {
            let refined = self.analyzer.analyze();

            if refined > 0 {
                return Ok(());
            }
        }

        Err(ProgressStalled{})
    }
}

/// A conflict error within a group.
#[derive(Clone, Debug)]
pub struct ConflictError {
    digit: Digit,
    candidate: CellIndex,
    conflicting: CellIndex,
}

impl fmt::Display for ConflictError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Cannot set {:?} at ({:?}, {:?}), for it conflicts with ({:?}, {:?})",
            self.digit, self.candidate.row(), self.candidate.column(), self.conflicting.row(), self.conflicting.column())
    }
}

impl error::Error for ConflictError {}

/// Progress is stalled.
#[derive(Clone, Debug)]
pub struct ProgressStalled {}

impl fmt::Display for ProgressStalled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Cannot progress any further")
    }
}

impl error::Error for ProgressStalled {}
