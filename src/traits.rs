use crate::iter::IndexedAccessIter;

/// Trait to index data
pub trait IndexedAccess {
    /// Inserts data into the file and returns its ID
    fn insert(&mut self, data: &[u8]) -> usize;

    /// Replaces an entry, given by its ID, with new data. Returns `None` if the position
    /// is out of bounds/does not exists
    fn replace(&mut self, pos: usize, data: &[u8]) -> Option<()>;

    /// Returns the data for a given item
    fn get(&self, pos: usize) -> Option<&[u8]>;

    /// Returns the data for given item without bound checking
    fn get_unchecked(&self, pos: usize) -> &[u8];

    /// Returns an iterator over all entries in the file
    #[inline]
    fn iter(&self) -> IndexedAccessIter<Self>
    where
        Self: Sized,
    {
        IndexedAccessIter::new(self)
    }

    /// Returns the amount of items in the file
    fn len(&self) -> usize;

    /// Returns true if the file is empty
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
