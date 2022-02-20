use crate::{backend::CacheBackend, Cache, CacheError, CacheKey, CacheResult};
use std::collections::hash_map::Entry;

use std::collections::HashMap;

/// Most simplistic implementation of an in-memory cache, with neither multithreading nor
/// capacity control.
pub type SimpleMemoryCache<T> = HashMap<CacheKey, T>;

impl<T: Cache> CacheBackend<T> for SimpleMemoryCache<T> {
    fn retrieve(&self, key: &CacheKey) -> CacheResult<Option<&T>> {
        Ok(self.get(key))
    }

    fn store(&mut self, key: &CacheKey, entity: T) -> CacheResult<()> {
        if let Entry::Vacant(entry) = self.entry(key.clone()) {
            entry.insert(entity);
            Ok(())
        } else {
            Err(CacheError::KeyAlreadyPresent(key.clone()))
        }
    }

    fn update(&mut self, key: &CacheKey, entity: T) -> CacheResult<()> {
        self.insert(key.clone(), entity);
        Ok(())
    }

    fn expire(&mut self, key: &CacheKey) -> CacheResult<()> {
        self.remove(key);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_KEY: CacheKey = 23;
    const TEST_ENTITY: u8 = 13;

    #[test]
    fn stores_entity_successfully() {
        let mut cache = SimpleMemoryCache::<u8>::new();

        assert!(cache.store(&TEST_KEY, TEST_ENTITY).is_ok());
    }

    #[test]
    fn fails_storing_already_existent_entity() {
        let mut cache = SimpleMemoryCache::<u8>::new();
        let _ = cache.store(&TEST_KEY, TEST_ENTITY);

        assert_eq!(
            cache.store(&TEST_KEY, TEST_ENTITY),
            Err(CacheError::KeyAlreadyPresent(TEST_KEY))
        );
    }

    #[test]
    fn retrieves_stored_entity() {
        let mut cache = SimpleMemoryCache::<u8>::new();
        let _ = cache.store(&TEST_KEY, TEST_ENTITY);

        assert_eq!(cache.retrieve(&TEST_KEY), Ok(Some(&TEST_ENTITY)));
    }

    #[test]
    fn retrieves_nothing_if_entity_not_present() {
        let mut cache = SimpleMemoryCache::<u8>::new();
        let _ = cache.store(&TEST_KEY, TEST_ENTITY);

        let non_present_key = 12;

        assert_eq!(cache.retrieve(&non_present_key), Ok(None));
    }

    #[test]
    fn updates_previously_stored_entity() {
        let mut cache = SimpleMemoryCache::<u8>::new();
        let _ = cache.store(&TEST_KEY, TEST_ENTITY);

        let new_entity = 92;

        assert!(cache.update(&TEST_KEY, new_entity).is_ok());
        assert_eq!(cache.retrieve(&TEST_KEY), Ok(Some(&new_entity)));
    }

    #[test]
    fn inserts_new_entity_when_trying_to_update_non_existent_one() {
        let mut cache = SimpleMemoryCache::<u8>::new();
        let _ = cache.store(&TEST_KEY, TEST_ENTITY);

        let new_key = 21;
        let new_entity = 92;

        assert!(cache.update(&new_key, new_entity).is_ok());
        assert_eq!(cache.retrieve(&new_key), Ok(Some(&new_entity)));
    }

    #[test]
    fn removes_previously_stored_entity() {
        let mut cache = SimpleMemoryCache::<u8>::new();
        let _ = cache.store(&TEST_KEY, TEST_ENTITY);

        assert!(cache.expire(&TEST_KEY).is_ok());
        assert_eq!(cache.retrieve(&TEST_KEY), Ok(None));
    }

    #[test]
    fn does_nothing_when_removing_non_existent_entity() {
        let mut cache = SimpleMemoryCache::<u8>::new();
        let _ = cache.store(&TEST_KEY, TEST_ENTITY);

        let new_key = 78;

        assert!(cache.expire(&new_key).is_ok());
    }
}
