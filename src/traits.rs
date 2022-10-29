use crate::iter::IndexedAccessIter;

#[cfg(feature = "typed")]
use serde::{de::DeserializeOwned, Serialize};

/// Trait to index data
pub trait IndexedAccessMut {
    /// Inserts data into the file and returns its ID
    fn insert(&mut self, data: &[u8]) -> usize;

    /// Replaces an entry, given by its ID, with new data. Returns `None` if the position
    /// is out of bounds/does not exists
    fn replace(&mut self, pos: usize, data: &[u8]) -> Option<()>;
}

/// Trait to index data
pub trait IndexedAccess {
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

#[cfg(feature = "typed")]
pub trait TypedIndexedAccessMut: IndexedAccessMut {
    #[inline]
    fn insert_typed<T: Serialize>(&mut self, item: &T) -> Result<usize, Box<bincode::ErrorKind>> {
        let enc = bincode::serialize(item)?;
        Ok(self.insert(&enc))
    }

    fn replace_typed<T: Serialize>(
        &mut self,
        pos: usize,
        new: &T,
    ) -> Result<bool, Box<bincode::ErrorKind>> {
        let new_enc = bincode::serialize(new)?;
        let res = self.replace(pos, &new_enc);
        Ok(res.is_some())
    }
}

#[cfg(feature = "typed")]
pub trait TypedIndexedAccess: IndexedAccess {
    #[inline]
    fn get_typed<T: DeserializeOwned>(
        &self,
        id: usize,
    ) -> Result<Option<T>, Box<bincode::ErrorKind>> {
        let data = self.get(id);
        if data.is_none() {
            return Ok(None);
        }

        let item: T = bincode::deserialize(data.unwrap())?;
        Ok(Some(item))
    }

    /// Returns an iterator over all entries in the file
    #[inline]
    fn iter_typed<T>(&self) -> crate::typed_iter::TypedIndexedAccessIter<Self, T>
    where
        Self: Sized,
        T: DeserializeOwned,
    {
        crate::typed_iter::TypedIndexedAccessIter::new(self)
    }
}

#[cfg(feature = "typed")]
impl<U: IndexedAccess> TypedIndexedAccess for U {}

#[cfg(feature = "typed")]
impl<U: IndexedAccessMut> TypedIndexedAccessMut for U {}
