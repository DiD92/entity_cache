use super::CacheBackend;
use crate::{Cache, CacheError, CacheKey};

use std::collections::HashMap;

pub type SimpleMemoryCache<T> = HashMap<CacheKey, T>;

impl<T: Cache> CacheBackend<T> for SimpleMemoryCache<T> {
    fn extract(&mut self, key: CacheKey) -> Option<T> {
        self.remove(&key)
    }

    fn store(&mut self, key: CacheKey, entity: T) -> Result<(), CacheError> {
        self.insert(key, entity);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_store() {
        let mut cache = SimpleMemoryCache::<u8>::new();

        let key = 23;

        assert!(cache.store(key, 13).is_ok());
    }
}
