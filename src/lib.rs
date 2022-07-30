pub mod iter;
pub mod memory;
pub mod traits;
pub mod vec;

#[cfg(feature = "mapped")]
pub mod map;

pub use memory::MemFile;
pub use vec::VecFile;

#[cfg(feature = "mapped")]
pub use map::MappedFile;
