use crate::{Cache, CacheKey, CacheResult};

mod standalone;

pub use standalone::SimpleMemoryCache;

pub trait CacheBackend<T: Cache> {
    /// Obtains entity from cache if present.
    fn retrieve(&self, key: &CacheKey) -> CacheResult<Option<&T>>;

    /// Stores entity in cache, if entity already present in cache
    /// the behaviour must be defined at the implementation level.
    fn store(&mut self, key: &CacheKey, entity: T) -> CacheResult<()>;

    /// Updates entity in cache, implementors can choose if they want to allow
    /// updating entities not present in the cache or not.
    fn update(&mut self, key: &CacheKey, entity: T) -> CacheResult<()>;

    /// Removes entity from cache
    fn expire(&mut self, key: &CacheKey) -> CacheResult<()>;
}
