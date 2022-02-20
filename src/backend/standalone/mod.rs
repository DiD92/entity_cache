///! This modules contains cache implementations meant to be used in a single machine.
mod memory;

// TODO: Something like pagefile?
// TODO: Investigate other options

pub use memory::SimpleMemoryCache;
