use crate::traits::{IndexedAccess, IndexedAccessMut};
use serde::{Deserialize, Serialize};
use std::ops::Index;

/// A vector based in-memory index. Not memory efficient at all.
/// Should only be used for testing purposes
#[derive(Clone, Serialize, Deserialize)]
pub struct VecFile {
    data: Vec<Vec<u8>>,
}

impl VecFile {
    #[inline]
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
}

impl IndexedAccessMut for VecFile {
    #[inline]
    fn insert(&mut self, data: &[u8]) -> usize {
        let id = self.data.len();
        self.data.push(data.to_vec());
        id
    }

    #[inline]
    fn replace(&mut self, pos: usize, data: &[u8]) -> Option<()> {
        *self.data.get_mut(pos)? = data.to_vec();
        Some(())
    }
}

impl IndexedAccess for VecFile {
    #[inline]
    fn get(&self, pos: usize) -> Option<&[u8]> {
        self.data.get(pos).map(|i| i.as_slice())
    }

    #[inline]
    fn get_unchecked(&self, pos: usize) -> &[u8] {
        unsafe { self.data.get_unchecked(pos).as_slice() }
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl Index<usize> for VecFile {
    type Output = [u8];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.get_unchecked(index)
    }
}
