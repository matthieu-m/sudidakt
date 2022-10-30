//! Incremental digit placer, based on analysis results.

use crate::model::{CellIndex, Digit};
use super::{JournalCursor, JournalReader, JournalWriter, Placement, PossibleValues, Refinement};

/// Incremental digit placer.
#[derive(Clone, Debug)]
pub struct Placer {
    possible_values: PossibleValues,
    placements: JournalWriter<Placement>,
    refinements: JournalCursor<Refinement>,
}

impl Placer {
    /// Creates an instance.
    pub fn new(placements: JournalWriter<Placement>, refinements: JournalReader<Refinement>) -> Self {
        let possible_values = PossibleValues::all();
        let refinements = JournalCursor::new(refinements);

        Self { possible_values, placements, refinements, }
    }

    /// Returns a handle over the placements.
    pub fn placements(&self) -> JournalReader<Placement> { self.placements.reader() }

    /// Returns whether the Placer is done with placements so far.
    pub fn is_done(&self) -> bool { self.refinements.is_done() }

    /// Manually set a digit, notably used for initialization.
    pub fn set_digit(&mut self, cell: CellIndex, digit: Digit) {
        #[cfg(debug_assertions)]
        eprintln!("Placer::set_digit - Place {digit:?} in {cell:?} ({:?}/{:?})", cell.row(), cell.column());

        self.possible_values.resolve(cell, digit);

        self.placements.append_event(Placement::new(cell, digit));
    }

    /// Incrementally process the next refinement.
    ///
    /// Returns the placement, if any was made.
    pub fn handle_next_refinement(&mut self) -> Option<Placement> {
        let possible_values = &mut self.possible_values;
        let placements = &self.placements;

        self.refinements.handle_next(|refinement| {
            let cell = refinement.cell();
            let digit = refinement.removed();

            #[cfg(debug_assertions)]
            eprintln!("Placer::handle_next_refinement - Remove {digit:?} from {cell:?} ({:?}/{:?}) => {:?}",
                cell.row(), cell.column(), possible_values.of_cell(cell));

            possible_values.remove_possibility(cell, digit)?;

            if possible_values.of_cell(cell).size() > 1 {
                return None;
            }

            //  The refinement has excluded all other possibilities for the given cell.
            let digit = possible_values.of_cell(cell).into_iter().next().unwrap();

            #[cfg(debug_assertions)]
            eprintln!("Placer::handle_next_refinement - Place {digit:?} in {cell:?} ({:?}/{:?})", cell.row(), cell.column());

            let placement = Placement::new(cell, digit);
            placements.append_event(placement);

            Some(placement)
        }).flatten()
    }
}
