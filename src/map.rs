use crate::{mem_index::MemIndex, traits::IndexedAccess};
use mmarinus::{perms, Map, Private};
use std::{
    fs::File,
    io::{BufReader, Error, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

/// A mmapped file
pub struct MappedFile {
    map: Map<perms::Read, Private>,
    path: PathBuf,
    len: usize,
    index: MemIndex,
}

impl MappedFile {
    /// Open a memory file mmapped
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let index = Self::read_index(path.as_ref())?;
        let map = Self::open_map(path.as_ref())?;
        let path = path.as_ref().to_path_buf();
        let len = File::open(&path)?.metadata()?.len() as usize;
        Ok(MappedFile {
            map,
            path,
            index,
            len,
        })
    }

    /// Cleans the mapping by closing the old file and reopening it
    pub fn reopen(&mut self) -> Result<(), Error> {
        self.map = Self::open_map(&self.path)?;
        Ok(())
    }

    /// Reloads the data index
    #[inline]
    pub fn reload_index(&mut self) -> Result<(), Error> {
        self.index = Self::read_index(&self.path)?;
        Ok(())
    }

    /// Decodes the index of a file
    fn read_index<P: AsRef<Path>>(path: P) -> Result<MemIndex, Error> {
        let mut reader = BufReader::new(File::open(path)?);
        let mut index_len_buf = [0u8; 8];
        reader.read_exact(&mut index_len_buf[..])?;
        let index_len = u64::from_le_bytes(index_len_buf);

        reader.seek(SeekFrom::Start(0))?;

        let to_read = index_len as usize * 4 + 8;
        let mut buf = vec![0u8; to_read];
        reader.read_exact(&mut buf[..])?;

        Ok(bincode::deserialize(&buf).unwrap())
    }

    /// Opens a file as Mapped file
    #[inline]
    fn open_map<P: AsRef<Path>>(path: P) -> Result<Map<perms::Read, Private>, Error> {
        let mut file = File::open(path)?;
        let size = file.metadata()?.len() as usize;
        let map = Map::bytes(size)
            .anywhere()
            .from(&mut file, 0)
            .with_kind(Private)
            .with(perms::Read)?;
        Ok(map)
    }
}

impl IndexedAccess for MappedFile {
    fn get(&self, pos: usize) -> Option<&[u8]> {
        let index_bytes = self.index.bytes_len();

        // self.len is tot-length with index
        // There are 8 bytes after index for the length of the encoded vec
        let data_end = self.len - index_bytes - 8;
        let range = self.index.index_item(pos, data_end)?;

        // Add index and data-vec-offset
        let data_offset = index_bytes + 8;

        let start = range.start + data_offset;
        let end = range.end + data_offset;

        Some(&self.map[start..end])
    }

    #[inline]
    fn get_unchecked(&self, pos: usize) -> &[u8] {
        self.get(pos).unwrap()
    }

    #[inline]
    fn len(&self) -> usize {
        self.index.len()
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufWriter};

    use crate::{
        traits::{TypedIndexedAccess, TypedIndexedAccessMut},
        MappedFile, MemFile,
    };

    #[test]
    fn test_open_mapped() {
        let mut mem = MemFile::new();
        for i in (0..337547u32).step_by(3) {
            mem.insert_typed(&i).unwrap();
        }

        let out = File::create("test_mapped_file_test").unwrap();
        let w = BufWriter::new(out);
        bincode::serialize_into(w, &mem).unwrap();

        let mapped = MappedFile::open("test_mapped_file_test").unwrap();

        assert_eq!(mapped.index.inner, mem.index.inner);

        let mut mapped_iter = mem.iter_typed::<u32>();
        for i in mapped.iter_typed::<u32>() {
            assert_eq!(mapped_iter.next(), Some(i));
        }

        let mut mapped_iter = mapped.iter_typed::<u32>();
        for i in mem.iter_typed::<u32>() {
            assert_eq!(mapped_iter.next(), Some(i));
        }

        std::fs::remove_file("test_mapped_file_test").unwrap();
    }
}
