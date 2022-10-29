use crate::traits::TypedIndexedAccess;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// Iterator over all entries of an indexed file
pub struct TypedIndexedAccessIter<'a, I, T> {
    file: &'a I,
    start: usize,
    end: usize,
    p: PhantomData<T>,
}

impl<'a, I, T> TypedIndexedAccessIter<'a, I, T>
where
    I: TypedIndexedAccess,
    T: DeserializeOwned,
{
    #[inline]
    pub(crate) fn new(file: &'a I) -> Self {
        // Last item
        let end = file.len();
        Self {
            file,
            start: 0,
            end,
            p: PhantomData,
        }
    }
}

impl<'a, I, T> ExactSizeIterator for TypedIndexedAccessIter<'a, I, T>
where
    I: TypedIndexedAccess,
    T: DeserializeOwned,
{
    #[inline]
    fn len(&self) -> usize {
        I::len(self.file)
    }
}

impl<'a, I, T> Iterator for TypedIndexedAccessIter<'a, I, T>
where
    I: TypedIndexedAccess,
    T: DeserializeOwned,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let content = self
            .file
            .get_typed(self.start)
            .expect("Failed to deserialize item in typed indexed iterator")?;
        self.start += 1;
        Some(content)
    }
}

impl<'a, I, T> DoubleEndedIterator for TypedIndexedAccessIter<'a, I, T>
where
    I: TypedIndexedAccess,
    T: DeserializeOwned,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == 0 {
            return None;
        }

        // we use 0 to indicate that the iterator is done
        let item = self
            .file
            .get_typed(self.end - 1)
            .expect("Failde to deserialize item in typed indexed iterator")?;
        self.end -= 1;
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{memory::MemFile, traits::TypedIndexedAccessMut, vec::VecFile};

    #[test]
    fn test_iter() {
        test(VecFile::new());
        test(MemFile::new());
    }

    // Generic func to test iterator for all implementations
    fn test<I: TypedIndexedAccess + TypedIndexedAccessMut>(mut idx: I) {
        let inp: Vec<_> = (1..10u32).collect();

        let mut ids = vec![];

        // Fill input
        for i in inp.iter() {
            ids.push(idx.insert_typed(i));
        }

        assert!(idx.len() > 0);
        assert_eq!(idx.iter().count(), inp.len());
        assert_eq!(idx.iter().rev().count(), inp.len());

        for (pos, data) in idx.iter_typed::<u32>().enumerate() {
            assert_eq!(inp[pos], data);
        }

        for (pos, data) in idx.iter_typed().rev().enumerate() {
            let real_pos = inp.len() - pos - 1;
            assert_eq!(inp[real_pos], data);
        }
    }
}
