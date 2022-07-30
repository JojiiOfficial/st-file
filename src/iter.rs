use crate::traits::IndexedAccess;

/// Iterator over all entries of an indexed file
pub struct IndexedAccessIter<'a, I> {
    file: &'a I,
    start: usize,
    end: usize,
}

impl<'a, I> IndexedAccessIter<'a, I>
where
    I: IndexedAccess,
{
    #[inline]
    pub(crate) fn new(file: &'a I) -> Self {
        // Last item
        let end = file.len();
        Self {
            file,
            start: 0,
            end,
        }
    }
}

impl<'a, I> ExactSizeIterator for IndexedAccessIter<'a, I>
where
    I: IndexedAccess,
{
    #[inline]
    fn len(&self) -> usize {
        I::len(self.file)
    }
}

impl<'a, I> Iterator for IndexedAccessIter<'a, I>
where
    I: IndexedAccess,
{
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let content = self.file.get(self.start)?;
        self.start += 1;
        Some(content)
    }
}

impl<'a, I> DoubleEndedIterator for IndexedAccessIter<'a, I>
where
    I: IndexedAccess,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == 0 {
            return None;
        }
        // we use 0 to indicate that the iterator is done
        let item = self.file.get(self.end - 1)?;
        self.end -= 1;
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{memory::MemFile, vec::VecFile};

    #[test]
    fn test_iter() {
        test(VecFile::new());
        test(MemFile::new());
    }

    // Generic func to test iterator for all implementations
    fn test<I: IndexedAccess>(mut idx: I) {
        let inp: Vec<_> = (1..10u32).map(|i| i.to_le_bytes()).collect();

        let mut ids = vec![];

        // Fill input
        for i in inp.iter() {
            ids.push(idx.insert(i));
        }

        assert!(idx.len() > 0);
        assert_eq!(idx.iter().count(), inp.len());
        assert_eq!(idx.iter().rev().count(), inp.len());

        for (pos, data) in idx.iter().enumerate() {
            let id = ids[pos];
            let exp = inp[pos];
            assert_eq!(data, exp);
            let get = idx.get(id).unwrap();
            assert_eq!(exp, get);
        }

        for (pos, data) in idx.iter().rev().enumerate() {
            let pos = idx.len() - pos - 1;
            let id = ids[pos];
            let exp = inp[pos];
            assert_eq!(data, exp);
            let get = idx.get(id).unwrap();
            assert_eq!(exp, get);
        }
    }
}
