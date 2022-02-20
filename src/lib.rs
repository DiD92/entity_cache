use std::error::Error;

mod backend;

pub type CacheError = Box<dyn Error>;

pub type CacheKey = usize;

// Marker trait for types that can be cached
pub trait Cache {}

impl<T> Cache for T where T: Copy {}
