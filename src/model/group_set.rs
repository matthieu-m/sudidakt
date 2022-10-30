//! A set of groups.

use std::{convert, fmt, iter};

use super::{DIMENSION, Group, GroupIndex};

/// Set of Group.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GroupSet(u64);

impl GroupSet {
    /// Creates an empty GroupSet.
    pub fn empty() -> GroupSet { GroupSet::default() }

    /// Creates a full GroupSet, with all values set.
    pub fn full() -> GroupSet { GroupSet((1 << NUMBER_GROUPS) - 1) }

    /// Checks whether the set is empty.
    pub fn is_empty(&self) -> bool { self.0 == 0 }

    /// Returns the number of elements in the set.
    pub fn size(&self) -> usize { self.0.count_ones() as usize }

    /// Checks whether the set contains the indicated Group.
    pub fn has(&self, group: Group) -> bool { (self.0 & Self::mask(group)) != 0 }

    /// Adds the specified Group.
    pub fn add(&mut self, group: Group) { self.0 |= Self::mask(group) }

    /// Removes the specified Group.
    pub fn remove(&mut self, group: Group) { self.0 &= !Self::mask(group) }

    //  Internal: computes the index of a group within the set.
    fn index(group: Group) -> usize { group.index().value() }

    //  Internal: computes the bitmask with the only set bit being that of the specified group.
    fn mask(group: Group) -> u64 { 1 << Self::index(group) }
}

impl convert::From<Group> for GroupSet {
    fn from(group: Group) -> GroupSet {
        let mut result = GroupSet::default();
        result.add(group);
        result
    }
}

impl fmt::Debug for GroupSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_set().entries(self.into_iter()).finish()
    }
}

impl iter::IntoIterator for GroupSet {
    type Item = Group;
    type IntoIter = GroupSetIterator;

    fn into_iter(self) -> Self::IntoIter { GroupSetIterator(self.0) }
}

/// Iterator over a set of Groups.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct GroupSetIterator(u64);

impl iter::Iterator for GroupSetIterator {
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let trailing = self.0.trailing_zeros();
        let mask = 1 << trailing;
        self.0 &= !mask;

        GroupIndex::new(trailing as usize).map(Group::new)
    }
}

//
//  Implementation
//

const NUMBER_GROUPS: usize = 3 * DIMENSION;

#[cfg(test)]
mod tests {

use super::*;

#[test]
fn empty_set() {
    let empty = GroupSet::default();

    assert!(empty.is_empty());
    assert_eq!(0, empty.size());
    assert_eq!("{}", &format!("{:?}", empty));
}

#[test]
fn single_group_set() {
    let single = GroupSet::from(group(3));

    assert!(!single.is_empty());
    assert_eq!(1, single.size());
    assert_eq!("{Column(3)}", &format!("{:?}", single));
}

#[test]
fn crud_group_set() {
    let three = group(3);
    let four = group(4);

    let mut set = GroupSet::default();
    set.add(three);

    assert!(set.has(three));
    assert!(!set.has(four));
    assert_eq!("{Column(3)}", &format!("{:?}", set));

    set.remove(four);

    assert!(set.has(three));
    assert!(!set.has(four));
    assert_eq!("{Column(3)}", &format!("{:?}", set));

    set.add(four);

    assert!(set.has(three));
    assert!(set.has(four));
    assert_eq!("{Column(3), Column(4)}", &format!("{:?}", set));

    set.remove(three);

    assert!(!set.has(three));
    assert!(set.has(four));
    assert_eq!("{Column(4)}", &format!("{:?}", set));

    set.remove(four);

    assert!(!set.has(three));
    assert!(!set.has(four));
    assert_eq!("{}", &format!("{:?}", set));
}

fn group(group: usize) -> Group { Group::new(GroupIndex::new(group).expect("Valid Group")) }

}
