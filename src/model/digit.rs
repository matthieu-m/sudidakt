//! A digit for the grid.

use std::{fmt, num::NonZeroU8};

use super::DIMENSION;

/// A `Digit` represents a single cell of a sudoku grid.
///
/// A `Digit` always falls in the `1..DIMENSION` range.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digit(NonZeroU8);

impl Digit {
    /// Creates a digit, if the number is within range.
    ///
    /// If the number is not within range, an error is returned.
    pub fn new(value: usize) -> Result<Digit, String> {
        if value > DIMENSION {
            return Err(format!("{} is not an appropriate digit for a {}x{} sudoku grid", value, DIMENSION, DIMENSION))
        }

        if let Some(non_zero) = NonZeroU8::new(value as u8) {
            Ok(Digit(non_zero))
        } else {
            Err(format!("0 is never an appropriate digit"))
        }
    }

    /// Returns the actual value of the digit.
    pub fn value(&self) -> usize { self.0.get().into() }
}

impl fmt::Debug for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {

use super::*;

#[test]
fn valid_digits() {
    for i in 1..=DIMENSION {
        let digit = Digit::new(i);
        assert_eq!(i, digit.expect("Valid Digit").value());
    }
}

#[test]
fn invalid_zero() {
    let result = Digit::new(0);
    assert_eq!(Err("0 is never an appropriate digit".to_string()), result);
}

#[test]
fn invalid_high() {
    let result = Digit::new(DIMENSION + 1);
    assert_eq!(Err("10 is not an appropriate digit for a 9x9 sudoku grid".to_string()), result);
}

}
