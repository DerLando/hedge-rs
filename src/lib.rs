//!
//! An index based half-edge mesh implementation.
//!

pub use crate::data::*;
pub use crate::handles::*;
pub use crate::traits::*;
//pub use crate::mesh::*;
//pub use crate::proxy::*;
//pub use crate::iterators::*;

pub mod data;
pub mod handles;
pub mod traits;
pub mod kernel;
//pub mod mesh;
//pub mod proxy;
//pub mod iterators;
