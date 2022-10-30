//! A counter for the number of occurrences of each digit.

use std::{fmt, iter};

use super::{DIMENSION, Digit};

/// Counter of Digits.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct DigitCounter([u8; DIMENSION]);

impl DigitCounter {
    /// Creates a full DigitCounter, for which each Digit has the maximum count.
    pub fn full() -> DigitCounter { DigitCounter([DIMENSION as u8; DIMENSION]) }

    /// Check the counter for a particular digit.
    pub fn count(&self, digit: Digit) -> usize { self.0[Self::index(digit)] as usize }

    /// Returns an iterator over the digits whose counter equals 1.
    pub fn singles(&self) -> SingleDigitCounterIterator { SingleDigitCounterIterator(self.0, 0) }

    /// Set the counter for a particular digit.
    ///
    /// #   Panics
    ///
    /// If the value is greater than or equal to `DIMENSION`.
    pub fn set(&mut self, digit: Digit, value: usize) {
        assert!(value < DIMENSION, "Cannot set counter for {:?}: {} >= {}", digit, value, DIMENSION);

        self.0[Self::index(digit)] = value as u8;
    }

    /// Decrement the counter for a particular digit.
    ///
    /// #   Panics
    ///
    /// If the counter value is already 0.
    pub fn decrement(&mut self, digit: Digit) {
        let counter = &mut self.0[Self::index(digit)];

        assert_ne!(0, *counter, "Cannot decrement counter of {:?}, it is already 0", digit);

        *counter -= 1;
    }

    /// Increment the counter for a particular digit.
    ///
    /// #   Panics
    ///
    /// If the counter value is already `DIMENSION - 1`.
    pub fn increment(&mut self, digit: Digit) {
        let counter = &mut self.0[Self::index(digit)];

        assert_ne!(DIMENSION - 1, *counter as usize,
            "Cannot increment counter of {:?}, it is already {}", digit, DIMENSION - 1);

        *counter += 1;
    }

    //  Internal: index of a digit.
    fn index(digit: Digit) -> usize { digit.value() - 1 }
}

impl fmt::Debug for DigitCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_map().entries(self.into_iter()).finish()
    }
}

impl iter::IntoIterator for DigitCounter {
    type Item = (Digit, usize);
    type IntoIter = DigitCounterIterator;

    fn into_iter(self) -> Self::IntoIter { DigitCounterIterator(self.0, 0) }
}

/// Iterator over Digits whose counter is 1.
pub struct SingleDigitCounterIterator([u8; DIMENSION], usize);

impl iter::Iterator for SingleDigitCounterIterator {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.1..DIMENSION {
            if self.0[i] == 1 {
                self.1 = i + 1;
                return Digit::new(i + 1).ok();
            }
        }

        self.1 = DIMENSION;
        None
    }
}

/// Iterator over all Digits
pub struct DigitCounterIterator([u8; DIMENSION], usize);

impl iter::Iterator for DigitCounterIterator {
    type Item = (Digit, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == DIMENSION {
            return None;
        }

        let index = self.1;
        self.1 += 1;

        Digit::new(index + 1)
            .ok()
            .map(|digit| (digit, self.0[index] as usize))
    }
}

#[cfg(test)]
mod tests {

use super::*;

#[test]
fn empty_counter() {
    let empty = DigitCounter::default();

    assert_eq!("{1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0, 9: 0}", format!("{:?}", empty));
}

}
