//! Driver

use std::{fs::File, io::BufReader, iter, ops::Range};

use sudidakt::model::{CellIndex, Digit, Grid};

mod automated;
mod interactive;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let (mode, problem) = parse_arguments(&args[..]);

    match mode {
        Mode::Interactive => interactive::run(problem.grid()),
        Mode::Automated => match problem {
            Problem::Immediate(grid) => automated::run(iter::once((0, grid))),
            Problem::Csv(filename, range, step) => automated::run(parse_csv(filename.as_str(), range, step)),
        },
    }
}

//
//  Argument parsing
//

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Mode {
    Interactive,
    Automated,
}

#[derive(Clone, Debug)]
enum Problem {
    Immediate(Grid),
    Csv(String, Range<usize>, usize),
}

impl Problem {
    fn grid(&self) -> Grid {
        match self {
            Problem::Immediate(grid) => grid.clone(),
            Problem::Csv(file, range, step) => parse_csv(file, range.clone(), *step).next().unwrap().1,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Options {
    csv: bool,
    line: Option<usize>,
    range: Option<Range<usize>>,
    step: Option<usize>,
}

fn parse_arguments(args: &[String]) -> (Mode, Problem) {
    let (mode, args) = consume_mode(&args[1..]);
    let (options, args) = consume_options(args);
    let positional = consume_positional(args);

    let problem = compute_problem(options, positional);

    if let Problem::Csv(_, range, _) = &problem {
        if mode == Mode::Interactive && range.end - range.start != 1 {
            eprintln!("A single problem at a time can be solved in interactive mode, use --line.");

            std::process::exit(1);
        }
    }

    (mode, problem)
}

fn print_help() -> ! {
    eprintln!("sudidakt [[i]nteractive|[a]utomated] [options] <grid/file>");
    eprintln!("");
    eprintln!("Meta:");
    eprintln!("\t-h|--help\tPrint this help and exit.");
    eprintln!("");
    eprintln!("Modes:");
    eprintln!("\tinteractive\tSolve the problem manually, with sudidakt checking and helping.");
    eprintln!("\tautomated\tLet sudidakt solve the problem(s).");
    eprintln!("");
    eprintln!("Problems:");
    eprintln!("\t-c/--csv\tTreat argument as filename, instead of grid.");
    eprintln!("\t-l/--line LINE\tUse the specified problem in the CSV.");
    eprintln!("\t-r/--rangeSTART END\tUse the specified range of problems in the CSV.");
    eprintln!("\t-s/--step STEP\tOnly process every STEP line in the CSV.");
    eprintln!("");
    eprintln!("The expected problem format is 81 characters left-to-right, top-to-bottom,");
    eprintln!("with zeros or dots for unknown digits. Spaces are ignored.");
    eprintln!("");
    eprintln!("The expected CSV format is an optional header, then one problem and");
    eprintln!("optionally its solution per row.");

    std::process::exit(1);
}

fn consume_mode(args: &[String]) -> (Mode, &[String]) {
    if args.is_empty() {
        print_help();
    }

    match args[0].as_str() {
        "i" | "interactive" => (Mode::Interactive, &args[1..]),
        "a" | "automated" => (Mode::Automated, &args[1..]),
        _ => {
            eprintln!("Unknown mode {}, expected [[i]nteractive|[a]utomated]", args[0]);

            std::process::exit(1);
        },
    }
}

fn consume_options(mut args: &[String]) -> (Options, &[String]) {
    fn parse_index(arg: Option<&String>, name: &str) -> usize {
        let arg = if let Some(arg) = arg {
            arg
        } else {
            eprintln!("{} expects an index as argument", name);

            std::process::exit(1);
        };

        if let Ok(arg) = arg.parse() {
            arg
        } else {
            eprintln!("{} expects a number as argument", name);

            std::process::exit(1);
        }
    }

    let mut options = Options::default();

    while let Some(arg) = args.first() {
        if !arg.starts_with('-') {
            break;
        }

        args = match arg.as_str() {
            "-c" | "--csv" => {
                options.csv = true;

                &args[1..]
            },
            "-l" | "--line" => {
                options.line = Some(parse_index(args.get(1), arg));

                &args[2..]
            },
            "-r" | "--range" => {
                options.range = Some(parse_index(args.get(1), arg)..parse_index(args.get(2), arg));

                &args[3..]
            },
            "-s" | "--step" => {
                options.step = Some(parse_index(args.get(1), arg));

                if options.step == Some(0) {
                    eprintln!("The --step option only supports strictly positive steps.");

                    std::process::exit(1);
                }

                &args[2..]
            }
            _ => print_help(),
        };
    }

    (options, args)
}

fn consume_positional(args: &[String]) -> &str {
    if args.len() != 1 {
        eprintln!("Expects one positional argument: the grid(s) or file.");

        std::process::exit(1);
    }

    &args[0]
}

fn compute_problem(options: Options, positional: &str) -> Problem {
    if !options.csv {
        if options.line.is_some() {
            eprintln!("The --line option is not supported without the --csv option.");

            std::process::exit(1);
        }

        if options.range.is_some() {
            eprintln!("The --range option is not supported without the --csv option.");

            std::process::exit(1);
        }

        if options.step.is_some() {
            eprintln!("The --step option is not supported without the --csv option.");

            std::process::exit(1);
        }

        return Problem::Immediate(parse_grid(positional));
    }

    let filename = String::from(positional);

    let range = match (options.line, options.range) {
        (None, None) => 0..usize::MAX,
        (Some(line), None) => line..(line+1),
        (None, Some(range)) => range,
        (Some(_), Some(_)) => {
            eprintln!("The --line option and the --range option are mutually exclusive.");

            std::process::exit(1);
        }
    };

    let step = options.step.unwrap_or(1);

    Problem::Csv(filename, range, step)
}

//
//  Grid parsing.
//

fn parse_grid(line: &str) -> Grid {
    let mut grid = Grid::default();

    let mut index = 0;

    for byte in line.bytes() {
        match byte {
            b'0' | b'.' => index += 1,
            b'1'..=b'9' => {
                let cell = CellIndex::new(index).expect("Valid index");
                let digit = Digit::new((byte - b'0') as usize).expect("1..=9");

                grid.set_digit(cell, Some(digit));

                index += 1;
            },
            b' ' => (),
            b',' => break,
            _ => panic!("Invalid character in grid definition: {}", byte),
        }
    }

    grid
}

//
//  Csv parsing.
//

fn parse_csv(filename: &str, range: Range<usize>, step: usize) -> impl Iterator<Item = (usize, Grid)> {
    use std::io::BufRead;

    let file = File::open(filename).expect("Csv file exists");

    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Valid line"))
        //  Skip header
        .skip_while(|line| {
            let first = line.as_bytes()[0];

            !first.is_ascii_digit() || first != b'.'
        })
        .enumerate()
        //  Skip not within range.
        .skip(range.start)
        //  Only take within range.
        .take(range.end - range.start)
        .step_by(step)
        .map(|(index, line)| (index, parse_grid(&line)))
}
