//!
//! An index based half-edge mesh implementation.
//!

pub use crate::traits::*;
pub use crate::handles::*;
pub use crate::data::*;
pub use crate::elements::*;
pub use crate::mesh::*;
pub use crate::proxy::*;
pub use crate::iterators::*;

pub mod traits;
pub mod handles;
pub mod data;
pub mod elements;
pub mod kernel;
pub mod proxy;
pub mod mesh;
//pub mod utils;
pub mod iterators;
