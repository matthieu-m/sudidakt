//! The Analyzer, which performs and keeps track of the various analyses.

use crate::model::{DIMENSION, SQUARE_DIMENSION, CellIndex, CellSet, Group, GroupSet};
use super::{ALL_ANALYSES, NUMBER_ANALYSIS, Analysis, JournalMultiCursor, JournalReader, JournalWriter, Placement, PossibleValues, Refinement, RefinementReason};

/// The Analyzer can incrementally perform the various analyses.
#[derive(Clone, Debug)]
pub struct Analyzer {
    //  The meta-data is owned, to ensure it is not modified behind the analyzer's back.
    possible_values: PossibleValues,
    //  Journal of refinements.
    refinements: JournalWriter<Refinement>,
    //  Cursor over placements.
    placements_cursors: JournalMultiCursor<Placement, NUMBER_CURSORS>,
    //  Cursor over refinements.
    refinements_cursors: JournalMultiCursor<Refinement, NUMBER_CURSORS>,
    //  Analyses.
    cell_exclusion: CellExclusion,
    group_exclusion: GroupExclusion,
    group_inclusion: GroupInclusion,
    group_overlap: GroupOverlap,
    group_subset_inclusion: GroupSubsetInclusion,
}

impl Analyzer {
    /// Creates an Analyzer.
    pub fn new(placements: JournalReader<Placement>) -> Self {
        let refinements = JournalWriter::new();
        let refinements_cursors = JournalMultiCursor::new(refinements.reader());

        Self {
            possible_values: PossibleValues::all(),
            refinements,
            placements_cursors: JournalMultiCursor::new(placements),
            refinements_cursors,
            cell_exclusion: CellExclusion,
            group_exclusion: GroupExclusion,
            group_inclusion: GroupInclusion,
            group_overlap: GroupOverlap,
            group_subset_inclusion: GroupSubsetInclusion,
        }
    }

    /// Returns the current possible values.
    pub fn possible_values(&self) -> PossibleValues { self.possible_values }

    /// Returns a handle over the refinements.
    pub fn refinements(&self) -> JournalReader<Refinement> { self.refinements.reader() }

    /// Returns whether the Analyzer is done with analyses so far.
    pub fn is_done(&self) -> bool {
        ALL_ANALYSES
            .into_iter()
            .map(Self::cursor_index)
            .all(|index| self.placements_cursors.is_done(index) && self.refinements_cursors.is_done(index))
    }

    /// Incrementally analyze placements and refinements until a refinement is produced or the analyzer has caught up
    /// with the end of the journals.
    ///
    /// Returns the number of refinements added, possibly 0 if no progress was made.
    pub fn analyze(&mut self) -> usize {
        for analysis in ALL_ANALYSES {
            let refined = self.analyze_next_placement_with(analysis);

            if refined != 0 {
                return refined;
            }

            let refined = self.analyze_next_refinement_with(analysis);

            if refined != 0 {
                return refined;
            }
        }

        0
    }

    /// Incrementally analyze one placement with the specified analysis strategy.
    ///
    /// Returns the number of refinements added, possibly 0 if no progress was made.
    pub fn analyze_next_placement_with(&mut self, analysis: Analysis) -> usize {
        let cursor_index = Self::cursor_index(analysis);
        let before = self.refinements_cursors.reader().len();

        let possible_values = &mut self.possible_values;
        let refinements = &self.refinements;

        let analysis: &mut dyn AnalysisImpl = match analysis {
            Analysis::CellExclusion => &mut self.cell_exclusion,
            Analysis::GroupExclusion => &mut self.group_exclusion,
            Analysis::GroupInclusion => &mut self.group_inclusion,
            Analysis::GroupOverlap => &mut self.group_overlap,
            Analysis::GroupSubsetInclusion => &mut self.group_subset_inclusion,
        };

        self.placements_cursors.handle_next(cursor_index,
            |placement| analysis.analyze_next_placement(possible_values, refinements, placement));

        self.refinements_cursors.reader().len() - before
    }

    /// Incrementally analyze one refinement with the specified analysis strategy.
    ///
    /// Returns the number of refinements added, possibly 0 if no progress was made.
    pub fn analyze_next_refinement_with(&mut self, analysis: Analysis) -> usize {
        let cursor_index = Self::cursor_index(analysis);
        let before = self.refinements_cursors.reader().len();

        let possible_values = &mut self.possible_values;
        let refinements = &self.refinements;

        let analysis: &mut dyn AnalysisImpl = match analysis {
            Analysis::CellExclusion => &mut self.cell_exclusion,
            Analysis::GroupExclusion => &mut self.group_exclusion,
            Analysis::GroupInclusion => &mut self.group_inclusion,
            Analysis::GroupOverlap => &mut self.group_overlap,
            Analysis::GroupSubsetInclusion => &mut self.group_subset_inclusion,
        };

        self.refinements_cursors.handle_next(cursor_index,
            |refinement| analysis.analyze_next_refinement(possible_values, refinements, refinement));

        self.refinements_cursors.reader().len() - before
    }
}

//
//  Implementation Details
//

const NUMBER_CURSORS: usize = NUMBER_ANALYSIS;

impl Analyzer {
    fn cursor_index(analysis: Analysis) -> usize { analysis as u8 as usize }
}

//
//  Analyses
//

trait AnalysisImpl {
    #[allow(unused_variables )]
    fn analyze_next_placement(
        &mut self,
        possible_values: &mut PossibleValues,
        refinements: &JournalWriter<Refinement>,
        placement: Placement,
    )
    {
    }

    #[allow(unused_variables )]
    fn analyze_next_refinement(
        &mut self,
        possible_values: &mut PossibleValues,
        refinements: &JournalWriter<Refinement>,
        refinement: Refinement,
    )
    {
    }
}

#[derive(Clone, Debug, Default)]
struct CellExclusion;

impl AnalysisImpl for CellExclusion {
    fn analyze_next_placement(
        &mut self,
        possible_values: &mut PossibleValues,
        refinements: &JournalWriter<Refinement>,
        placement: Placement,
    )
    {
        let cell = placement.cell();
        let digit = placement.digit();

        //  Set cell value.
        let removed_digits = possible_values.resolve(cell, digit);

        #[cfg(debug_assertions)]
        eprintln!("CellExclusion::analyze - Remove {:?} from {cell:?} ({:?}/{:?})", removed_digits, cell.row(), cell.column());

        for removed in removed_digits {
            refinements.append_event(Refinement::new(cell, removed, RefinementReason::CellExclusion(digit)));
        }
    }
}

#[derive(Clone, Debug, Default)]
struct GroupExclusion;

impl AnalysisImpl for GroupExclusion {
    fn analyze_next_placement(
        &mut self,
        possible_values: &mut PossibleValues,
        refinements: &JournalWriter<Refinement>,
        placement: Placement,
    )
    {
        let cell = placement.cell();
        let digit = placement.digit();

        //  Exclude digit from all cells sharing a group with the current cell.
        for group in Group::groups(cell) {
            for other in group.cells() {
                if other == cell {
                    continue;
                }

                if let Some(digit) = possible_values.remove_possibility(other, digit) {
                    #[cfg(debug_assertions)]
                    eprintln!("GroupExclusion::analyze - Remove {digit:?} from {cell:?} ({:?}/{:?})", cell.row(), cell.column());

                    refinements.append_event(Refinement::new(other, digit, RefinementReason::GroupExclusion(cell, group)));
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct GroupInclusion;

impl AnalysisImpl for GroupInclusion {
    fn analyze_next_refinement(
        &mut self,
        possible_values: &mut PossibleValues,
        refinements: &JournalWriter<Refinement>,
        refinement: Refinement,
    )
    {
        let cell = refinement.cell();
        let digit = refinement.removed();

        for group in Group::groups(cell) {
            if possible_values.of_group(group).count(digit) != 1 {
                continue;
            }

            let candidate = group.cells().into_iter()
                .filter(|candidate| possible_values.of_cell(*candidate).has(digit))
                .next()
                .expect("1 cell, as per the count above");

            for removed in possible_values.resolve(candidate, digit) {
                #[cfg(debug_assertions)]
                eprintln!("GroupInclusion::analyze - Remove {:?} from {cell:?} ({:?}/{:?})", removed, cell.row(), cell.column());

                let reason = RefinementReason::GroupInclusion(digit, group);
                refinements.append_event(Refinement::new(candidate, removed, reason));
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct GroupOverlap;

impl AnalysisImpl for GroupOverlap {
    fn analyze_next_refinement(
        &mut self,
        possible_values: &mut PossibleValues,
        refinements: &JournalWriter<Refinement>,
        refinement: Refinement,
    )
    {
        let cell = refinement.cell();
        let digit = refinement.removed();

        let groups = Group::groups(cell);

        for includer in groups {
            if possible_values.of_group(includer).count(digit) > SQUARE_DIMENSION {
                //  Cannot possibly have all overlapping with a single other groups in other 2 group types.
                continue;
            }

            let mut overlapping_a = GroupSet::empty();
            let mut overlapping_b = GroupSet::empty();

            for candidate in includer.cells() {
                if possible_values.of_cell(candidate).has(digit) {
                    let [a, b] = includer.other_groups(candidate);

                    overlapping_a.add(a);
                    overlapping_b.add(b);
                }
            }

            if overlapping_a.is_empty() {
                //  This should never happen, for the digit must be present in at least one cell of each group.
                continue;
            }

            for overlapping_set in [overlapping_a, overlapping_b] {
                if overlapping_set.size() > 1 {
                    //  Overlapping with too many of this group type, impossible to know which it'll end up in.
                    continue;
                }

                let overlapping_group = overlapping_set.into_iter().next().expect("At least one group");

                for overlapping_cell in overlapping_group.cells() {
                    if includer.contains(overlapping_cell) {
                        continue;
                    }

                    if possible_values.of_cell(overlapping_cell).has(digit) {
                        #[cfg(debug_assertions)]
                        eprintln!("GroupOverlap::analyze - Remove {digit:?} from {overlapping_cell:?} ({:?}/{:?})",
                            overlapping_cell.row(), overlapping_cell.column());

                        let reason = RefinementReason::GroupOverlap(includer, overlapping_group);

                        possible_values.remove_possibility(overlapping_cell, digit);
                        refinements.append_event(Refinement::new(overlapping_cell, digit, reason));
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct GroupSubsetInclusion;

impl GroupSubsetInclusion {
    //  Search for all cells with the same subset of possible digits as the argument.
    //
    //  #   Complexity
    //
    //  Quadratic (time) in the number of cells per group.
    fn analyze_next_cell(
        &mut self,
        possible_values: &mut PossibleValues,
        refinements: &JournalWriter<Refinement>,
        cell: CellIndex,
    )
    {
        let digits_subset = possible_values.of_cell(cell);

        //  GroupInclusion will handle that case very well, and cheaper.
        if digits_subset.size() <= 1 {
            return;
        }

        //  Let's limit to subsets of 2, 3, or 4 elements, to avoid complexity running away from us.
        if digits_subset.size() > DIMENSION / 2 {
            return;
        }

        for group in Group::groups(cell) {
            let mut cells_subset = CellSet::from(cell);

            for candidate in group.cells() {
                if candidate == cell {
                    continue;
                }

                if possible_values.of_cell(candidate).is_subset_of(&digits_subset) {
                    cells_subset.add(candidate);
                }
            }

            if cells_subset.size() != digits_subset.size() {
                continue;
            }

            for candidate in group.cells() {
                if cells_subset.has(candidate) {
                    continue;
                }

                for digit in digits_subset {
                    if possible_values.of_cell(candidate).has(digit) {
                        #[cfg(debug_assertions)]
                        eprintln!("GroupSubsetInclusion::analyze - Remove {digit:?} from {candidate:?} ({:?}/{:?})",
                            candidate.row(), candidate.column());

                        let reason = RefinementReason::GroupSubsetInclusion(cells_subset, digits_subset, group);

                        possible_values.remove_possibility(candidate, digit);
                        refinements.append_event(Refinement::new(candidate, digit, reason));
                    }
                }
            }
        }
    }
}

impl AnalysisImpl for GroupSubsetInclusion {
    fn analyze_next_refinement(
        &mut self,
        possible_values: &mut PossibleValues,
        refinements: &JournalWriter<Refinement>,
        refinement: Refinement,
    )
    {
        //  There are two possibilities for running the analysis:
        //
        //  -   Based on the cell: quadratic.
        //  -   Based on the cells still containing the removed digit: cubic.

        let cell = refinement.cell();
        let digit = refinement.removed();

        self.analyze_next_cell(possible_values, refinements, cell);

        for group in Group::groups(cell) {
            for cell in group.cells() {
                if possible_values.of_cell(cell).has(digit) {
                    self.analyze_next_cell(possible_values, refinements, cell);
                }
            }
        }
    }
}
