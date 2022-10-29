use std::ops::Range;

use serde::{Deserialize, Serialize};

/// In memory index for data offsets
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MemIndex {
    pub(crate) inner: Vec<u32>,
}

impl MemIndex {
    #[inline]
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    /// Inserts a new item to the index and returns its ID
    #[inline]
    pub fn insert(&mut self, data_offset: u32) -> usize {
        let id = self.next_id();
        self.inner.push(data_offset);
        id
    }

    /// Gets the index item for the given ID
    #[inline]
    pub fn get(&self, id: usize) -> Option<u32> {
        self.inner.get(id).copied()
    }

    /// Returns the index range of the data between `id` and id+1 (or end) if
    /// `pos` is the last item
    #[inline]
    pub fn index_item(&self, id: usize, end: usize) -> Option<Range<usize>> {
        let start = *self.inner.get(id)? as usize;

        let next_id = id + 1;

        if !self.has_id(next_id) {
            return Some(start..end);
        }

        let next = unsafe { *self.inner.get_unchecked(next_id) } as usize;

        Some(start..next)
    }

    /// Same as [`index_item`] but doesn't check bounds. The caller has to ensure that
    /// `pos` is within the index and `end` correctly points to the last element of the data
    #[inline]
    pub unsafe fn index_item_unchecked(&self, id: usize, end: usize) -> (usize, usize) {
        let start = *self.inner.get_unchecked(id) as usize;
        let next_id = id + 1;
        if self.has_id(next_id) {
            (start, *self.inner.get(next_id).unwrap() as usize)
        } else {
            (start, end)
        }
    }

    /// Returns `true` if the given ID is in the index
    #[inline]
    pub fn has_id(&self, id: usize) -> bool {
        id < self.inner.len()
    }

    /// Applies the given delta to all entries after `from_pos` (including)
    #[inline]
    pub fn update_range(&mut self, from_pos: usize, delta: isize) {
        for i in self.inner.iter_mut().skip(from_pos) {
            *i = (*i as isize + delta) as u32;
        }
    }

    /// Returns the id of the next item
    #[inline]
    pub fn next_id(&self) -> usize {
        self.inner.len()
    }

    /// Returns the amount of items in the index
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub(crate) fn bytes_len(&self) -> usize {
        (self.len() * 4) + 8
    }
}

impl From<Vec<u32>> for MemIndex {
    #[inline]
    fn from(inner: Vec<u32>) -> Self {
        Self { inner }
    }
}

impl Default for MemIndex {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
