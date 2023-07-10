use crate::{
    mem_index::MemIndex,
    traits::{IndexedAccess, IndexedAccessMut},
};
use serde::{Deserialize, Serialize};
use std::ops::Index;

/// An In-memory indexable "file" that allows inserting, getting and replacing
/// variable length [u8] arrays using an ID.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MemFile {
    pub(crate) index: MemIndex,
    pub(crate) data: Vec<u8>,
}

impl IndexedAccessMut for MemFile {
    #[inline]
    fn insert(&mut self, data: &[u8]) -> usize {
        let id = self.index.insert(self.data.len() as u32);
        self.data.extend_from_slice(data);
        id
    }

    #[inline]
    fn replace(&mut self, pos: usize, data: &[u8]) -> Option<()> {
        let range = self.index.index_item(pos, self.raw_len())?;

        // Replace data
        self.data.splice(range.clone(), data.iter().copied());

        // Update index with new positions in since those could've been changed
        let diff = data.len() as isize - range.len() as isize;

        self.index.update_range(pos + 1, diff);

        Some(())
    }
}

impl IndexedAccess for MemFile {
    #[inline]
    fn get(&self, pos: usize) -> Option<&[u8]> {
        let range = self.index.index_item(pos, self.raw_len())?;
        Some(&self.data[range])
    }

    #[inline]
    fn get_unchecked(&self, pos: usize) -> &[u8] {
        let (start, end) = unsafe { self.index.index_item_unchecked(pos, self.raw_len()) };
        &self.data[start..end]
    }

    #[inline]
    fn len(&self) -> usize {
        self.index.len()
    }
}

impl MemFile {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            index: MemIndex::new(),
        }
    }

    #[inline]
    pub fn new_raw(data: Vec<u8>, index: Vec<u32>) -> Self {
        Self {
            data,
            index: MemIndex::from(index),
        }
    }

    /// Returns the amount of bytes stored in the file
    #[inline]
    pub fn raw_len(&self) -> usize {
        self.data.len()
    }
}

impl<I: AsRef<[u8]>> Extend<I> for MemFile {
    #[inline]
    fn extend<T: IntoIterator<Item = I>>(&mut self, iter: T) {
        for line in iter {
            self.insert(line.as_ref());
        }
    }
}

impl<U: Iterator<Item = impl AsRef<[u8]>>> From<U> for MemFile {
    #[inline]
    fn from(iter: U) -> Self {
        let mut new = MemFile::new();
        for i in iter {
            new.insert(i.as_ref());
        }
        new
    }
}

impl Index<usize> for MemFile {
    type Output = [u8];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get_unchecked(index)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::read_to_string};

    use super::*;

    fn test_data() -> &'static [&'static str] {
        &[
            "俺はプログラミングできねええ",
            "音楽好き",
            "昨日のコーヒー飲んじゃった",
            "this is a text",
        ]
    }

    #[test]
    fn test_mem_file_unicode() {
        test_entries(test_data());
    }

    #[test]
    fn test_replace() {
        let mut m_file = MemFile::new();
        let data = test_data();
        for entry in data {
            m_file.insert(entry.as_bytes());
        }

        for (pos, i) in data.iter().enumerate() {
            assert_eq!(m_file.get(pos).unwrap(), i.as_bytes());
        }

        m_file.replace(0, "lol".as_bytes()).unwrap();
        assert_eq!(m_file.get(0), Some("lol".as_bytes()));
        for (pos, i) in data.iter().enumerate().skip(1) {
            assert_eq!(m_file.get(pos).unwrap(), i.as_bytes());
        }

        m_file.replace(0, "sometesttextあぶ".as_bytes()).unwrap();
        assert_eq!(m_file.get(0), Some("sometesttextあぶ".as_bytes()));
        for (pos, i) in data.iter().enumerate().skip(1) {
            assert_eq!(m_file.get(pos).unwrap(), i.as_bytes());
        }

        m_file.replace(0, data[0].as_bytes()).unwrap();
        for (pos, i) in data.iter().enumerate() {
            assert_eq!(m_file.get(pos).unwrap(), i.as_bytes());
        }

        m_file
            .replace(m_file.len() - 1, "lastlol".as_bytes())
            .unwrap();
        for (pos, i) in data.iter().enumerate().rev().skip(1) {
            assert_eq!(m_file.get(pos).unwrap(), i.as_bytes());
        }
        assert_eq!(m_file.get(m_file.len() - 1).unwrap(), "lastlol".as_bytes());
    }

    #[test]
    fn test_mem_file_2() {
        let input_files = &["simple", "LICENSE", "input1"];

        for input_file in input_files {
            let file = format!("./testfiles/{}", input_file);
            let content = read_to_string(file).unwrap();

            let split: Vec<_> = content.split('\n').collect();
            test_entries(&split);
            test_from_iter(&split);

            let split: Vec<_> = content.split(' ').collect();
            test_entries(&split);
            test_from_iter(&split);
        }
    }

    fn test_entries(entries: &[&str]) {
        let mut new_file = MemFile::new();
        let mut map = HashMap::new();

        for (pos, entry) in entries.iter().enumerate() {
            let file_pos = new_file.insert(entry.as_bytes());
            map.insert(pos, file_pos);
        }

        assert_eq!(new_file.len(), entries.len());

        for (entry_pos, file_pos) in map {
            let entry = entries[entry_pos];
            let raw = new_file.get(file_pos).unwrap();
            let file_str = std::str::from_utf8(raw).unwrap();
            assert_eq!(file_str, entry);
        }

        for (res, exp) in new_file.iter().zip(entries.iter()) {
            let res_str = std::str::from_utf8(res).unwrap();
            assert_eq!(res_str, *exp);
        }
    }

    fn test_from_iter(entries: &[&str]) {
        let new_file = MemFile::from(entries.iter());

        assert_eq!(new_file.len(), entries.len());

        for (pos, entry) in entries.iter().enumerate() {
            let raw = new_file.get(pos).unwrap();
            let file_str = std::str::from_utf8(raw).unwrap();
            assert_eq!(file_str, *entry);
        }
    }
}
