#![deny(warnings)]

pub mod accu;
mod core;
pub mod graph;
pub mod io;
pub mod morpho;
pub mod structures;

pub use core::*;

// Tests
#[cfg(test)]
mod tests;
