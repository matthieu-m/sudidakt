//! Automated run of sudidakt.

use std::time::Instant;

use sudidakt::{
    model::{CellIndex, ColumnIndex, Digit, Grid, RowIndex},
    solver::{PossibleValues, Solver},
};

/// Runs the automated mode.
pub fn run(iterator: impl Iterator<Item = (usize, Grid)>) {
    for (index, grid) in iterator {
        println!("Solving grid {index}");

        let start = Instant::now();

        let mut solver = Solver::new(grid);

        if let Err(e) = solver.solve() {
            println!("Failed to solve grid {index}: {}", e);
            println!("");

            println!("Start grid:");
            display_grid(&grid);
            println!("");

            println!("Current grid:");
            display_grid(&solver.grid());
            println!("");

            println!("Possible values:");
            display_possible_values(&solver.possible_values());
            println!("");

            return;
        }

        println!("Solved grid {index} in {:?}", start.elapsed());
    }
}

//
//  Display
//

fn display_grid(grid: &Grid) {
    use std::fmt::Write;

    for row in RowIndex::all() {
        if row.value() == 3 || row.value() == 6 {
            println!("");
        }

        let mut formatted = String::from("    ");

        for column in ColumnIndex::all() {
            if column.value() == 3 || column.value() == 6 {
                formatted.push(' ');
            }

            let cell = CellIndex::from_coordinates(row, column);

            if let Some(digit) = grid.get_digit(cell) {
                write!(formatted, "{}", digit.value()).unwrap();
            } else {
                formatted.push('.');
            }
        }

        println!("{}", formatted);
    }
}

fn display_possible_values(values: &PossibleValues) {
    use std::fmt::Write;

    for row in RowIndex::all() {
        if row.value() == 3 || row.value() == 6 {
            println!("    ____________|_____________|____________");
        }

        if row.value() != 0 {
            println!("                |             |");
        }

        let (mut zero, mut one, mut two) = (String::from("    "), String::from("    "), String::from("    "));

        for column in ColumnIndex::all() {
            if column.value() > 0 {
                zero.push(' ');
                one.push(' ');
                two.push(' ');
            }

            if column.value() == 3 || column.value() == 6 {
                zero.push_str("| ");
                one.push_str("| ");
                two.push_str("| ");
            }

            let cell = CellIndex::from_coordinates(row, column);

            let values = values.of_cell(cell);

            for i in 0..3 {
                let z = Digit::new(i + 1).unwrap();
                let o = Digit::new(i + 1 + 3).unwrap();
                let t = Digit::new(i + 1 + 6).unwrap();

                if values.has(z) {
                    write!(zero, "{}", i + 1).unwrap();
                } else {
                    zero.push('.');
                }

                if values.has(o) {
                    write!(one, "{}", i + 1 + 3).unwrap();
                } else {
                    one.push('.');
                }

                if values.has(t) {
                    write!(two, "{}", i + 1 + 6).unwrap();
                } else {
                    two.push('.');
                }
            }
        }

        println!("{}", zero);
        println!("{}", one);
        println!("{}", two);
    }
}
