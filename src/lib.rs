mod backend;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CacheError {
    KeyAlreadyPresent(CacheKey),
}

pub type CacheResult<T> = Result<T, CacheError>;

pub type CacheKey = usize;

// Marker trait for types that can be cached
pub trait Cache {}

impl<T> Cache for T where T: Clone {}
