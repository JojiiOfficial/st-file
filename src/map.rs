// Unfinished! Maybe don't make this mutable but rather allow converting between MemFile and MappedFile for mutable access
use memmap2::MmapMut;
use std::{fs::File, io::Write, path::Path};

use crate::traits::IndexedAccess;

pub struct MappedFile {
    map: MmapMut,
}

impl MappedFile {
    /// Creates a new MappedFile
    #[inline]
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = File::create(path)?;
        Self::from_file(&file)
    }

    /// Opens an existing MappedFile
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        Self::from_file(&file)
    }

    #[inline]
    fn from_file(file: &File) -> Result<Self, std::io::Error> {
        let map = unsafe { MmapMut::map_mut(file)? };
        Ok(Self { map })
    }

    /// Writes data to the file
    fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        (&mut self.map[..]).write_all(data)?;
        Ok(())
    }
}

impl IndexedAccess for MappedFile {
    fn insert(&mut self, data: &[u8]) -> usize {
        todo!()
    }

    fn replace(&mut self, pos: usize, data: &[u8]) -> Option<()> {
        todo!()
    }

    fn get(&self, pos: usize) -> Option<&[u8]> {
        todo!()
    }

    fn get_unchecked(&self, pos: usize) -> &[u8] {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}
