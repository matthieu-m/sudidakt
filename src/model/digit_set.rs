//! A set of digits.

use std::{convert, fmt, iter};

use super::{DIMENSION, Digit};

/// Set of Digits.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DigitSet(u16);

impl DigitSet {
    /// Creates a full DigitSet, with all values set.
    pub fn full() -> DigitSet { DigitSet((1 << DIMENSION) - 1) }

    /// Checks whether the set is empty.
    pub fn is_empty(&self) -> bool { self.0 == 0 }

    /// Returns the number of elements in the set.
    pub fn size(&self) -> usize { self.0.count_ones() as usize}

    /// Returns whether the set contains the indicated Digit.
    pub fn has(&self, digit: Digit) -> bool { (self.0 & Self::mask(digit)) != 0 }

    /// Returns whether the set is a subset of the argument.
    pub fn is_subset_of(&self, other: &DigitSet) -> bool { self.0 | other.0 == other.0 }

    /// Returns whether the set is a superset of the argument.
    pub fn is_superset_of(&self, other: &DigitSet) -> bool { other.is_subset_of(self) }

    /// Adds the specified Digit.
    pub fn add(&mut self, digit: Digit) { self.0 |= Self::mask(digit) }

    /// Removes the specified Digit.
    pub fn remove(&mut self, digit: Digit) { self.0 &= !Self::mask(digit) }

    //  Internal: computes the index of a digit within the set.
    fn index(digit: Digit) -> usize { digit.value() - 1 }

    //  Internal: computes the bitmask with the only set bit being that of the specified digit.
    fn mask(digit: Digit) -> u16 { 1 << Self::index(digit) }
}

impl convert::From<Digit> for DigitSet {
    fn from(digit: Digit) -> DigitSet {
        let mut result = DigitSet::default();
        result.add(digit);
        result
    }
}

impl fmt::Debug for DigitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_set().entries(self.into_iter()).finish()
    }
}

impl iter::IntoIterator for DigitSet {
    type Item = Digit;
    type IntoIter = DigitSetIterator;

    fn into_iter(self) -> Self::IntoIter { DigitSetIterator(self.0) }
}

/// Iterator over a set of Digits.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct DigitSetIterator(u16);

impl iter::Iterator for DigitSetIterator {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let trailing = self.0.trailing_zeros();
        let mask = 1 << trailing;
        self.0 &= !mask;

        Digit::new(trailing as usize + 1).ok()
    }
}

#[cfg(test)]
mod tests {

use super::*;

#[test]
fn empty_set() {
    let empty = DigitSet::default();

    assert!(empty.is_empty());
    assert_eq!(0, empty.size());
    assert_eq!("{}", &format!("{:?}", empty));
}

#[test]
fn single_digit_set() {
    let single = DigitSet::from(digit(3));

    assert!(!single.is_empty());
    assert_eq!(1, single.size());
    assert_eq!("{3}", &format!("{:?}", single));
}

#[test]
fn crud_digit_set() {
    let three = digit(3);
    let four = digit(4);

    let mut set = DigitSet::default();
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

fn digit(digit: usize) -> Digit { Digit::new(digit).expect("Valid Digit") }

}
