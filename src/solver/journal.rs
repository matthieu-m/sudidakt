//! A generic journal facility.

use std::{cell::RefCell, rc::Rc};

/// A reader of the journal.
///
/// The reader can only, well, read.
#[derive(Debug)]
pub struct JournalReader<T>(Rc<RefCell<Vec<T>>>);

impl<T> JournalReader<T> {
    /// Returns the size of the journal, so far.
    pub fn len(&self) -> usize { self.0.borrow().len() }
}

impl<T: Clone> JournalReader<T> {
    /// Returns a clone of the event at the specified index, if any.
    pub fn get_event(&self, index: usize) -> Option<T> { self.0.borrow().get(index).cloned() }

    /// Returns a clone of _all_ the events.
    pub fn get_events(&self) -> Vec<T> { self.0.borrow().clone() }
}

impl<T> Clone for JournalReader<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

/// Single cursor over a journal reader.
#[derive(Debug)]
pub struct JournalCursor<T> {
    reader: JournalReader<T>,
    cursor: usize,
}

impl<T> JournalCursor<T> {
    /// Creates an instance.
    pub fn new(reader: JournalReader<T>) -> Self { Self { reader, cursor: 0, } }

    /// Returns a handle to the reader.
    pub fn reader(&self) -> &JournalReader<T> { &self.reader }

    /// Returns whether the cursor has processed all elements so far.
    pub fn is_done(&self) -> bool { self.cursor == self.reader.len() }
}

impl<T: Clone> JournalCursor<T> {
    /// Handles the next event, and marks it as such.
    ///
    /// Returns the result of the handler, or None if the handler was not invoked, that is if there was no event to
    /// handle.
    ///
    /// #   Panics
    ///
    /// -   If the handler panics.
    ///
    /// In case of panic, the event is not considered handled.
    pub fn handle_next<H, R>(&mut self, handler: H) -> Option<R>
    where
        H: FnOnce(T) -> R,
    {
        let index = self.cursor;
        let event = self.reader.get_event(index)?;

        let result = handler(event);

        self.cursor = index + 1;

        Some(result)
    }
}

impl<T> Clone for JournalCursor<T> {
    fn clone(&self) -> Self { Self { reader: self.reader.clone(), cursor: self.cursor, } }
}

/// Multiple cursors over a journal reader, for memory reasons.
#[derive(Debug)]
pub struct JournalMultiCursor<T, const N: usize> {
    reader: JournalReader<T>,
    cursors: [usize; N],
}

impl<T, const N: usize> JournalMultiCursor<T, N> {
    /// Creates an instance.
    pub fn new(reader: JournalReader<T>) -> Self { Self { reader, cursors: [0; N], } }

    /// Returns a handle to the reader.
    pub fn reader(&self) -> &JournalReader<T> { &self.reader }

    /// Returns whether the cursor has processed all elements so far.
    pub fn is_done(&self, cursor: usize) -> bool { self.cursors[cursor] == self.reader.len() }
}

impl<T: Clone, const N: usize> JournalMultiCursor<T, N> {
    /// Handles the next event for a specific cursor, and marks it as such.
    ///
    /// Returns the result of the handler, or None if the handler was not invoked, that is if there was no event to
    /// handle.
    ///
    /// #   Panics
    ///
    /// -   If the cursor index is greater than or equal to N.
    /// -   If the handler panics.
    ///
    /// In case of panic, the event is not considered handled.
    pub fn handle_next<H, R>(&mut self, cursor: usize, handler: H) -> Option<R>
    where
        H: FnOnce(T) -> R,
    {
        let index = self.cursors[cursor];
        let event = self.reader.get_event(index)?;

        let result = handler(event);

        self.cursors[cursor] = index + 1;

        Some(result)
    }
}

impl<T, const N: usize> Clone for JournalMultiCursor<T, N> {
    fn clone(&self) -> Self { Self { reader: self.reader.clone(), cursors: self.cursors, } }
}

/// A writer to the journal.
///
/// The writer can both read and append events.
#[derive(Debug)]
pub struct JournalWriter<T>(Rc<RefCell<Vec<T>>>);

impl<T> JournalWriter<T> {
    /// Creates a JournalWriter.
    pub fn new() -> Self { Self(Rc::new(RefCell::default())) }

    /// Creates a JournalReader for the current writer.
    pub fn reader(&self) -> JournalReader<T> { JournalReader(self.0.clone()) }

    /// Appends an event to the journal.
    pub fn append_event(&self, event: T) { self.0.borrow_mut().push(event) }
}

impl<T> Clone for JournalWriter<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T> Default for JournalWriter<T> {
    fn default() -> Self { Self::new() }
}
