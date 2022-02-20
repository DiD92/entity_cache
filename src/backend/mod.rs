use crate::{Cache, CacheError, CacheKey};

mod memory;

pub use memory::SimpleMemoryCache;

pub trait CacheBackend<T: Cache> {
    fn extract(&mut self, key: CacheKey) -> Option<T>;

    fn store(&mut self, key: CacheKey, entity: T) -> Result<(), CacheError>;
}
