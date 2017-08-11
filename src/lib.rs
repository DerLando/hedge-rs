
//!
//! An index based half-edge mesh implementation.
//!

#[macro_use]
extern crate log;
extern crate cgmath;

pub use components::*;
pub use mesh::*;
pub use iterators::*;
pub use operations::*;
pub use function_sets::*;

pub mod components;
pub mod mesh;
pub mod operations;
pub mod iterators;
pub mod function_sets;


#[cfg(test)]
mod tests;
